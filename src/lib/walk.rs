//! This module provides support for walking the file system.
use anyhow::{Error, Result};
use ignore::{overrides::OverrideBuilder, WalkBuilder};

use crate::args::Args;
use crate::stdstreams::format_print_file;
use crate::types::SizTypesBuilder;

pub struct Walker {
    walker: ignore::Walk,
}

impl Walker {
    pub fn new(args: &Args) -> Result<Self> {
        // we unwrap Option here because we know it is Some(PathBuf) from
        // the arg parsing logic in main.rs
        let mut binding = WalkBuilder::new(args.path.as_ref().unwrap());
        let walker = binding
            .hidden(!args.hidden)
            .skip_stdout(true)
            .require_git(true)
            .max_depth(args.depth)
            .git_global(false)
            .git_exclude(false)
            .follow_links(false);

        // sort by file path string
        if args.name {
            walker.sort_by_file_path(|a, b| a.cmp(b));
        }

        // filter files on user-defined default types
        // Note: This is not compatible with the glob option defined below.
        //       We do not allow both options to be used together at arg parse
        //       time.
        match &args.default_type {
            Some(user_types) => {
                let mut types_builder = SizTypesBuilder::new();
                walker.types(types_builder.filter_types(user_types)?);
            }
            None => (),
        }

        // filter files on user-defined globs
        // Note: This is not compatible with the default_type option defined above.
        //       We do not allow both options to be used together at arg parse time.
        match &args.glob {
            Some(globs) => {
                if !globs.is_empty() {
                    // we unwrap Option here because we know it is Some(PathBuf) from
                    // the arg parsing logic in main.rs
                    let mut ovrb = OverrideBuilder::new(args.path.as_ref().unwrap());
                    for glob in globs {
                        ovrb.add(glob)?;
                    }
                    let ovr = ovrb.build()?;
                    // add the overrides to the walker
                    walker.overrides(ovr);
                }
            }
            None => (),
        }

        Ok(Self {
            walker: walker.build(),
        })
    }
}

impl Iterator for Walker {
    type Item = Result<ignore::DirEntry>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.walker.next() {
            Some(Ok(entry)) => Some(Ok(entry)),
            Some(Err(err)) => Some(Err(Error::new(err))),
            None => None,
        }
    }
}

impl From<FileWalker> for Walker {
    fn from(item: FileWalker) -> Self {
        Walker {
            walker: item.walker,
        }
    }
}

pub struct FileWalker {
    walker: ignore::Walk,
}

impl FileWalker {
    pub fn new(args: &Args) -> Result<Self> {
        let walker = Walker::new(args)?;
        Ok(walker.into())
    }
}

impl Iterator for FileWalker {
    type Item = Result<ignore::DirEntry>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.walker.by_ref().find(|entry| match entry {
            // filter on file paths only, exclude all directory paths
            Ok(entry) => entry.path().is_file(),
            Err(_) => false,
        }) {
            Some(Ok(entry)) => Some(Ok(entry)),
            Some(Err(err)) => Some(Err(Error::new(err))),
            None => None,
        }
    }
}

impl From<Walker> for FileWalker {
    fn from(item: Walker) -> Self {
        FileWalker {
            walker: item.walker,
        }
    }
}

pub struct ParallelWalker {
    pub walker: ignore::WalkParallel,
}

impl ParallelWalker {
    pub fn new(args: &Args) -> Result<Self> {
        // we unwrap Option here because we know it is Some(PathBuf) from
        // the arg parsing logic in main.rs
        let mut binding = WalkBuilder::new(args.path.as_ref().unwrap());
        let walker = binding
            .hidden(!args.hidden)
            .skip_stdout(true)
            .require_git(true)
            .max_depth(args.depth)
            .git_global(false)
            .git_exclude(false)
            .follow_links(false);

        // filter files on user-defined default types
        // Note: This is not compatible with the glob option defined below.
        //       We do not allow both options to be used together at arg parse
        //       time.
        match &args.default_type {
            Some(default_types) => {
                let mut types_builder = SizTypesBuilder::new();
                let types = types_builder.filter_types(default_types)?;
                walker.types(types);
            }
            None => (),
        }

        // filter files on user-defined globs
        // Note: This is not compatible with the default_type option defined above.
        //       We do not allow both options to be used together at arg parse time.
        match &args.glob {
            Some(globs) => {
                if !globs.is_empty() {
                    // we unwrap Option here because we know it is Some(PathBuf) from
                    // the arg parsing logic in main.rs
                    let mut ovrb = OverrideBuilder::new(args.path.as_ref().unwrap());
                    for glob in globs {
                        ovrb.add(glob)?;
                    }
                    let ovr = ovrb.build()?;
                    // add the overrides to the walker
                    walker.overrides(ovr);
                }
            }
            None => (), // ignore, this should never happen b/c it is prohibited on the CL
        }

        Ok(Self {
            walker: walker.build_parallel(),
        })
    }

    pub fn print_files(
        self,
        args: &Args,
        metric_size_formatter: impl Fn(u64) -> String + Send + std::marker::Sync,
        binary_size_formatter: impl Fn(u64) -> String + Send + std::marker::Sync,
    ) -> Result<()> {
        self.walker.run(|| {
            Box::new(|entry| match entry {
                Ok(entry) => {
                    // filter on file paths only, exclude all directory paths
                    if entry.path().is_file() {
                        match entry.metadata() {
                            Ok(metadata) => match format_print_file(
                                args,
                                &metadata.len(),
                                entry.path(),
                                &metric_size_formatter,
                                &binary_size_formatter,
                            ) {
                                Ok(_) => ignore::WalkState::Continue,
                                Err(err) => {
                                    let mut walk_state = ignore::WalkState::Quit;
                                    let aerr = anyhow::Error::new(err);
                                    let mut broken_pipe_error = false;
                                    for cause in aerr.chain() {
                                        if let Some(ioerr) = cause.downcast_ref::<std::io::Error>()
                                        {
                                            if ioerr.kind() == std::io::ErrorKind::BrokenPipe {
                                                walk_state = ignore::WalkState::Continue;
                                                broken_pipe_error = true;
                                                break;
                                            }
                                        }
                                    }
                                    if !broken_pipe_error {
                                        eprintln!("Error printing to standard output: {}", aerr);
                                    }
                                    walk_state
                                }
                            },
                            Err(e) => {
                                eprintln!("Error reading metadata: {}", e);
                                ignore::WalkState::Quit
                            }
                        }
                    } else {
                        // is a directory, not a file
                        // continue
                        ignore::WalkState::Continue
                    }
                }
                Err(e) => {
                    eprintln!("Error reading entry: {}", e);
                    ignore::WalkState::Quit
                }
            })
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use ignore::{DirEntry, WalkParallel, WalkState};
    use pretty_assertions::assert_eq;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::sync::{Arc, Mutex};
    use tempfile::TempDir;

    fn tmpdir() -> TempDir {
        TempDir::new().unwrap()
    }

    fn mkdir_on_path<P: AsRef<Path>>(path: P) {
        std::fs::create_dir_all(path).unwrap();
    }

    fn write_file<P: AsRef<Path>>(path: P, contents: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }

    // fn write_file_with_size<P: AsRef<Path>>(path: P, size: u64) {
    //     let file = File::create(path).unwrap();
    //     file.set_len(size).unwrap();
    // }

    fn normalize_path(unix: &str) -> String {
        if cfg!(windows) {
            unix.replace("\\", "/")
        } else {
            unix.to_string()
        }
    }

    fn mkpaths(paths: &[&str]) -> Vec<String> {
        let paths: Vec<_> = paths.iter().map(|s| s.to_string()).collect();
        paths
    }

    fn walk_file_collect(prefix: &Path, args: &Args) -> Result<Vec<String>> {
        let mut paths = vec![];
        for result in FileWalker::new(args)? {
            let dirent = match result {
                Err(_) => continue,
                Ok(dirent) => dirent,
            };
            let path = dirent.path().strip_prefix(prefix).unwrap();
            if path.as_os_str().is_empty() {
                continue;
            }
            paths.push(normalize_path(path.to_str().unwrap()));
        }

        Ok(paths)
    }

    fn walk_file_collect_sorted(prefix: &Path, args: &Args) -> Result<Vec<String>> {
        let mut paths = vec![];
        for result in FileWalker::new(args)? {
            let dirent = match result {
                Err(_) => continue,
                Ok(dirent) => dirent,
            };
            let path = dirent.path().strip_prefix(prefix).unwrap();
            if path.as_os_str().is_empty() {
                continue;
            }
            paths.push(normalize_path(path.to_str().unwrap()));
        }
        paths.sort();
        Ok(paths)
    }

    fn walk_collect_parallel(prefix: &Path, args: &Args) -> Result<Vec<String>> {
        let mut paths = vec![];
        for dirent in walk_collect_entries_parallel(ParallelWalker::new(args)?.walker) {
            let path = dirent.path().strip_prefix(prefix).unwrap();
            if path.as_os_str().is_empty() {
                continue;
            }
            paths.push(normalize_path(path.to_str().unwrap()));
        }
        // sort the paths before returning in order
        // in order to be able to test. This represents
        // an artificial order in the results, but we will
        // still be able to confirm we have complete results
        paths.sort();
        Ok(paths)
    }

    fn walk_collect_entries_parallel(par_walker: WalkParallel) -> Vec<DirEntry> {
        let dirents = Arc::new(Mutex::new(vec![]));
        par_walker.run(|| {
            let dirents = dirents.clone();
            Box::new(move |result| {
                if let Ok(dirent) = result {
                    dirents.lock().unwrap().push(dirent);
                }
                WalkState::Continue
            })
        });

        let dirents = dirents.lock().unwrap();
        dirents.to_vec()
    }

    // fn assert_paths_sequential(prefix: &Path, args: &Args, expected: &[&str]) -> Result<()> {
    //     let got = walk_collect(prefix, args)?;
    //     assert_eq!(got, mkpaths(expected), "single threaded");
    //     Ok(())
    // }

    fn assert_file_paths_sequential(prefix: &Path, args: &Args, expected: &[&str]) -> Result<()> {
        let got = walk_file_collect(prefix, args)?;
        assert_eq!(got, mkpaths(expected), "single threaded, files only");
        Ok(())
    }

    fn assert_file_paths_sequential_sorted(
        prefix: &Path,
        args: &Args,
        expected: &[&str],
    ) -> Result<()> {
        let got = walk_file_collect_sorted(prefix, args)?;
        assert_eq!(got, mkpaths(expected), "single threaded, files only");
        Ok(())
    }

    fn assert_paths_parallel_sorted(prefix: &Path, args: &Args, expected: &[&str]) -> Result<()> {
        let got = walk_collect_parallel(prefix, args)?;
        assert_eq!(got, mkpaths(expected), "parallel");
        Ok(())
    }

    // ==================
    // Default execution
    // ==================
    #[test]
    fn test_walker_default() -> Result<()> {
        let td = tmpdir();
        let td_path = td.path().to_str().unwrap();
        mkdir_on_path(td.path().join("a/b/c"));
        mkdir_on_path(td.path().join("y/z"));
        write_file(td.path().join("a/b/.hide.txt"), "");
        write_file(td.path().join("a/b/ack.js"), "");
        write_file(td.path().join("a/b/foo.txt"), "");
        write_file(td.path().join("a/b/zoo.txt"), "");
        write_file(td.path().join("a/b/zoo.py"), "");
        write_file(td.path().join("a/b/zip.py"), "");
        write_file(td.path().join("y/z/foo.md"), "");
        write_file(td.path().join("y/z/.hide2.txt"), "");

        let args = Args::parse_from(vec!["siz", td_path]);

        assert_file_paths_sequential_sorted(
            td.path(),
            &args,
            &[
                "a/b/ack.js",
                "a/b/foo.txt",
                "a/b/zip.py",
                "a/b/zoo.py",
                "a/b/zoo.txt",
                "y/z/foo.md",
            ],
        )?;

        assert_paths_parallel_sorted(
            td.path(),
            &args,
            &[
                "a",
                "a/b",
                "a/b/ack.js",
                "a/b/c",
                "a/b/foo.txt",
                "a/b/zip.py",
                "a/b/zoo.py",
                "a/b/zoo.txt",
                "y",
                "y/z",
                "y/z/foo.md",
            ],
        )?;

        Ok(())
    }

    // =====================
    // --name sorted option
    // =====================
    #[test]
    fn test_walker_name() -> Result<()> {
        let td = tmpdir();
        let td_path = td.path().to_str().unwrap();
        mkdir_on_path(td.path().join("a/b/c"));
        mkdir_on_path(td.path().join("y/z"));
        write_file(td.path().join("a/b/.hide.txt"), "");
        write_file(td.path().join("a/b/ack.js"), "");
        write_file(td.path().join("a/b/foo.txt"), "");
        write_file(td.path().join("a/b/zoo.txt"), "");
        write_file(td.path().join("a/b/zoo.py"), "");
        write_file(td.path().join("a/b/zip.py"), "");
        write_file(td.path().join("y/z/foo.md"), "");

        let args = Args::parse_from(vec!["siz", "--name", td_path]);

        // preserve walker name output sorting here
        assert_file_paths_sequential(
            td.path(),
            &args,
            &[
                "a/b/ack.js",
                "a/b/foo.txt",
                "a/b/zip.py",
                "a/b/zoo.py",
                "a/b/zoo.txt",
                "y/z/foo.md",
            ],
        )?;

        // no parallel support for this option

        Ok(())
    }

    // =========================
    // --hidden filter option
    // ========================
    #[test]
    fn test_walker_hidden() -> Result<()> {
        let td = tmpdir();
        let td_path = td.path().to_str().unwrap();
        mkdir_on_path(td.path().join("a/b/c"));
        mkdir_on_path(td.path().join("y/z"));
        write_file(td.path().join("a/b/.hide.txt"), "");
        write_file(td.path().join("a/b/ack.js"), "");
        write_file(td.path().join("a/b/foo.txt"), "");
        write_file(td.path().join("a/b/zoo.txt"), "");
        write_file(td.path().join("a/b/zoo.py"), "");
        write_file(td.path().join("a/b/zip.py"), "");
        write_file(td.path().join("y/z/foo.md"), "");

        let args = Args::parse_from(vec!["siz", "--hidden", td_path]);

        assert_file_paths_sequential_sorted(
            td.path(),
            &args,
            &[
                "a/b/.hide.txt", // here is the hidden file
                "a/b/ack.js",
                "a/b/foo.txt",
                "a/b/zip.py",
                "a/b/zoo.py",
                "a/b/zoo.txt",
                "y/z/foo.md",
            ],
        )?;

        assert_paths_parallel_sorted(
            td.path(),
            &args,
            &[
                "a",
                "a/b",
                "a/b/.hide.txt", // here is the hidden file
                "a/b/ack.js",
                "a/b/c",
                "a/b/foo.txt",
                "a/b/zip.py",
                "a/b/zoo.py",
                "a/b/zoo.txt",
                "y",
                "y/z",
                "y/z/foo.md",
            ],
        )?;

        Ok(())
    }

    // ================================
    // --hidden & --name filter option
    // ================================
    #[test]
    fn test_walker_hidden_name() -> Result<()> {
        let td = tmpdir();
        let td_path = td.path().to_str().unwrap();
        mkdir_on_path(td.path().join("a/b/c"));
        mkdir_on_path(td.path().join("y/z"));
        write_file(td.path().join("a/b/.hide.txt"), "");
        write_file(td.path().join("a/b/ack.js"), "");
        write_file(td.path().join("a/b/foo.txt"), "");
        write_file(td.path().join("a/b/zoo.txt"), "");
        write_file(td.path().join("a/b/zoo.py"), "");
        write_file(td.path().join("a/b/zip.py"), "");
        write_file(td.path().join("y/z/foo.md"), "");

        let args = Args::parse_from(vec!["siz", "--hidden", "--name", td_path]);

        // preserve walker output sorting here
        assert_file_paths_sequential(
            td.path(),
            &args,
            &[
                "a/b/.hide.txt", // here is the hidden file
                "a/b/ack.js",
                "a/b/foo.txt",
                "a/b/zip.py",
                "a/b/zoo.py",
                "a/b/zoo.txt",
                "y/z/foo.md",
            ],
        )?;

        // no parallel support for this option combination

        Ok(())
    }

    // ================================
    // --depth recursion depth
    // ================================
    #[test]
    fn test_walker_depth() -> Result<()> {
        let td = tmpdir();
        let td_path = td.path().to_str().unwrap();
        mkdir_on_path(td.path().join("a/b/c"));
        mkdir_on_path(td.path().join("y/z"));
        write_file(td.path().join(".hide1.txt"), "");
        write_file(td.path().join("a1.txt"), "");
        write_file(td.path().join("a1.py"), "");
        write_file(td.path().join("z1.txt"), "");
        write_file(td.path().join("a/.hide2.txt"), "");
        write_file(td.path().join("a/a2.rs"), "");
        write_file(td.path().join("a/z2.rs"), "");
        write_file(td.path().join("a/b2.js"), "");
        write_file(td.path().join("a/b2.txt"), "");
        write_file(td.path().join("a/b/.hide3.txt"), "");
        write_file(td.path().join("a/b/z3.txt"), "");
        write_file(td.path().join("a/b/z3.py"), "");
        write_file(td.path().join("a/b/a3.py"), "");
        write_file(td.path().join("y/z/a3.md"), "");

        let args1 = Args::parse_from(vec!["siz", "--depth", "1", td_path]);
        let args2 = Args::parse_from(vec!["siz", "--depth", "2", td_path]);
        let args3 = Args::parse_from(vec!["siz", "--depth", "3", td_path]);

        // test traversal depth = 1, sequential
        assert_file_paths_sequential_sorted(td.path(), &args1, &["a1.py", "a1.txt", "z1.txt"])?;

        // test traversal depth = 2, sequential
        assert_file_paths_sequential_sorted(
            td.path(),
            &args2,
            &[
                "a/a2.rs", "a/b2.js", "a/b2.txt", "a/z2.rs", "a1.py", "a1.txt", "z1.txt",
            ],
        )?;

        // test traversal depth = 3, sequential
        assert_file_paths_sequential_sorted(
            td.path(),
            &args3,
            &[
                "a/a2.rs",
                "a/b/a3.py",
                "a/b/z3.py",
                "a/b/z3.txt",
                "a/b2.js",
                "a/b2.txt",
                "a/z2.rs",
                "a1.py",
                "a1.txt",
                "y/z/a3.md",
                "z1.txt",
            ],
        )?;

        // test traversal depth = 1, parallel
        assert_paths_parallel_sorted(td.path(), &args1, &["a", "a1.py", "a1.txt", "y", "z1.txt"])?;

        // test traversal depth = 2, parallel
        assert_paths_parallel_sorted(
            td.path(),
            &args2,
            &[
                "a", "a/a2.rs", "a/b", "a/b2.js", "a/b2.txt", "a/z2.rs", "a1.py", "a1.txt", "y",
                "y/z", "z1.txt",
            ],
        )?;

        // test traversal depth = 3, parallel
        assert_paths_parallel_sorted(
            td.path(),
            &args3,
            &[
                "a",
                "a/a2.rs",
                "a/b",
                "a/b/a3.py",
                "a/b/c",
                "a/b/z3.py",
                "a/b/z3.txt",
                "a/b2.js",
                "a/b2.txt",
                "a/z2.rs",
                "a1.py",
                "a1.txt",
                "y",
                "y/z",
                "y/z/a3.md",
                "z1.txt",
            ],
        )?;

        Ok(())
    }

    // ======================================
    // --depth recursion depth with --hidden
    // ======================================
    #[test]
    fn test_walker_depth_hidden() -> Result<()> {
        let td = tmpdir();
        let td_path = td.path().to_str().unwrap();
        mkdir_on_path(td.path().join("a/b/c"));
        mkdir_on_path(td.path().join("y/z"));
        write_file(td.path().join(".hide1.txt"), "");
        write_file(td.path().join("a1.txt"), "");
        write_file(td.path().join("a1.py"), "");
        write_file(td.path().join("z1.txt"), "");
        write_file(td.path().join("a/.hide2.txt"), "");
        write_file(td.path().join("a/a2.rs"), "");
        write_file(td.path().join("a/z2.rs"), "");
        write_file(td.path().join("a/b2.js"), "");
        write_file(td.path().join("a/b2.txt"), "");
        write_file(td.path().join("a/b/.hide3.txt"), "");
        write_file(td.path().join("a/b/z3.txt"), "");
        write_file(td.path().join("a/b/z3.py"), "");
        write_file(td.path().join("a/b/a3.py"), "");
        write_file(td.path().join("y/z/a3.md"), "");

        let args1 = Args::parse_from(vec!["siz", "--hidden", "--depth", "1", td_path]);
        let args2 = Args::parse_from(vec!["siz", "--hidden", "--depth", "2", td_path]);
        let args3 = Args::parse_from(vec!["siz", "--hidden", "--depth", "3", td_path]);

        // test traversal depth = 1, sequential
        assert_file_paths_sequential_sorted(
            td.path(),
            &args1,
            &[".hide1.txt", "a1.py", "a1.txt", "z1.txt"],
        )?;

        // test traversal depth = 2, sequential
        assert_file_paths_sequential_sorted(
            td.path(),
            &args2,
            &[
                ".hide1.txt",
                "a/.hide2.txt",
                "a/a2.rs",
                "a/b2.js",
                "a/b2.txt",
                "a/z2.rs",
                "a1.py",
                "a1.txt",
                "z1.txt",
            ],
        )?;

        // test traversal depth = 3, sequential
        assert_file_paths_sequential_sorted(
            td.path(),
            &args3,
            &[
                ".hide1.txt",
                "a/.hide2.txt",
                "a/a2.rs",
                "a/b/.hide3.txt",
                "a/b/a3.py",
                "a/b/z3.py",
                "a/b/z3.txt",
                "a/b2.js",
                "a/b2.txt",
                "a/z2.rs",
                "a1.py",
                "a1.txt",
                "y/z/a3.md",
                "z1.txt",
            ],
        )?;

        // test traversal depth = 1, parallel
        assert_paths_parallel_sorted(
            td.path(),
            &args1,
            &[".hide1.txt", "a", "a1.py", "a1.txt", "y", "z1.txt"],
        )?;

        // test traversal depth = 2, parallel
        assert_paths_parallel_sorted(
            td.path(),
            &args2,
            &[
                ".hide1.txt",
                "a",
                "a/.hide2.txt",
                "a/a2.rs",
                "a/b",
                "a/b2.js",
                "a/b2.txt",
                "a/z2.rs",
                "a1.py",
                "a1.txt",
                "y",
                "y/z",
                "z1.txt",
            ],
        )?;

        // test traversal depth = 3, parallel
        assert_paths_parallel_sorted(
            td.path(),
            &args3,
            &[
                ".hide1.txt",
                "a",
                "a/.hide2.txt",
                "a/a2.rs",
                "a/b",
                "a/b/.hide3.txt",
                "a/b/a3.py",
                "a/b/c",
                "a/b/z3.py",
                "a/b/z3.txt",
                "a/b2.js",
                "a/b2.txt",
                "a/z2.rs",
                "a1.py",
                "a1.txt",
                "y",
                "y/z",
                "y/z/a3.md",
                "z1.txt",
            ],
        )?;

        Ok(())
    }
}
