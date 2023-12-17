use anyhow::{Error, Result};
use ignore::{overrides::OverrideBuilder, WalkBuilder};

use crate::args::Args;

pub struct Walker {
    walker: ignore::Walk,
}

impl Walker {
    pub fn new(args: &Args) -> Result<Self> {
        let mut binding = WalkBuilder::new(&args.path);
        let walker = binding.hidden(!args.hidden).skip_stdout(true);

        // sort by file path string
        if args.name {
            walker.sort_by_file_path(|a, b| a.cmp(b));
        }

        // filter files on user-defined globs
        match &args.glob {
            Some(globs) => {
                if !globs.is_empty() {
                    let mut ovrb = OverrideBuilder::new(&args.path);
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
            // filter on file paths only, exclude directory paths
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
        let mut binding = WalkBuilder::new(&args.path);
        let walker = binding.hidden(!args.hidden).skip_stdout(true);

        // filter files on user-defined globs
        match &args.glob {
            Some(globs) => {
                if !globs.is_empty() {
                    let mut ovrb = OverrideBuilder::new(&args.path);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    // use ignore::{DirEntry, WalkParallel, WalkState};
    use pretty_assertions::assert_eq;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    // use std::sync::{Arc, Mutex};
    use tempfile::TempDir;

    fn mk_args(
        path: &Path,
        glob: Option<Vec<std::string::String>>,
        hidden: bool,
        highlow: bool,
        name: bool,
        parallel: bool,
    ) -> Args {
        Args {
            path: path.to_path_buf(),
            binary_units: false, // does not influence tests here
            color: false,        // does not influence tests here
            glob,
            hidden,
            highlow,
            metric_units: false, // does not influence tests here
            name,
            parallel,
        }
    }

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

    // fn walk_collect(prefix: &Path, args: &Args) -> Result<Vec<String>> {
    //     let mut paths = vec![];
    //     for result in Walker::new(args)? {
    //         let dirent = match result {
    //             Err(_) => continue,
    //             Ok(dirent) => dirent,
    //         };
    //         let path = dirent.path().strip_prefix(prefix).unwrap();
    //         if path.as_os_str().is_empty() {
    //             continue;
    //         }
    //         paths.push(normalize_path(path.to_str().unwrap()));
    //     }

    //     Ok(paths)
    // }

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

    // fn walk_collect_parallel(prefix: &Path, args: &Args) -> Result<Vec<String>> {
    //     let mut paths = vec![];
    //     for dirent in walk_collect_entries_parallel(ParallelWalker::new(args)?.walker) {
    //         let path = dirent.path().strip_prefix(prefix).unwrap();
    //         if path.as_os_str().is_empty() {
    //             continue;
    //         }
    //         paths.push(normalize_path(path.to_str().unwrap()));
    //     }
    //     // sort the paths before returning in order
    //     // in order to be able to test. This represents
    //     // an artificial order in the results, but we will
    //     // still be able to confirm we have complete results
    //     paths.sort();
    //     Ok(paths)
    // }

    // fn walk_collect_entries_parallel(par_walker: WalkParallel) -> Vec<DirEntry> {
    //     let dirents = Arc::new(Mutex::new(vec![]));
    //     par_walker.run(|| {
    //         let dirents = dirents.clone();
    //         Box::new(move |result| {
    //             if let Ok(dirent) = result {
    //                 dirents.lock().unwrap().push(dirent);
    //             }
    //             WalkState::Continue
    //         })
    //     });

    //     let dirents = dirents.lock().unwrap();
    //     dirents.to_vec()
    // }

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

    // fn assert_paths_parallel(prefix: &Path, args: &Args, expected: &[&str]) -> Result<()> {
    //     let got = walk_collect_parallel(prefix, args)?;
    //     assert_eq!(got, mkpaths(expected), "parallel");
    //     Ok(())
    // }

    // ==================
    // Default execution
    // ==================
    #[test]
    fn test_walker_default() -> Result<()> {
        let td = tmpdir();
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

        let args = mk_args(td.path(), None, false, false, false, false);

        assert_file_paths_sequential(
            td.path(),
            &args,
            &[
                "a/b/ack.js",
                "a/b/zoo.py",
                "a/b/zip.py",
                "a/b/foo.txt",
                "a/b/zoo.txt",
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
        mkdir_on_path(td.path().join("a/b/c"));
        mkdir_on_path(td.path().join("y/z"));
        write_file(td.path().join("a/b/.hide.txt"), "");
        write_file(td.path().join("a/b/ack.js"), "");
        write_file(td.path().join("a/b/foo.txt"), "");
        write_file(td.path().join("a/b/zoo.txt"), "");
        write_file(td.path().join("a/b/zoo.py"), "");
        write_file(td.path().join("a/b/zip.py"), "");
        write_file(td.path().join("y/z/foo.md"), "");

        let args = mk_args(td.path(), None, false, false, true, false);

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
        mkdir_on_path(td.path().join("a/b/c"));
        mkdir_on_path(td.path().join("y/z"));
        write_file(td.path().join("a/b/.hide.txt"), "");
        write_file(td.path().join("a/b/ack.js"), "");
        write_file(td.path().join("a/b/foo.txt"), "");
        write_file(td.path().join("a/b/zoo.txt"), "");
        write_file(td.path().join("a/b/zoo.py"), "");
        write_file(td.path().join("a/b/zip.py"), "");
        write_file(td.path().join("y/z/foo.md"), "");

        let args = mk_args(td.path(), None, true, false, false, false);

        assert_file_paths_sequential(
            td.path(),
            &args,
            &[
                "a/b/.hide.txt", // here is the hidden file
                "a/b/ack.js",
                "a/b/zoo.py",
                "a/b/zip.py",
                "a/b/foo.txt",
                "a/b/zoo.txt",
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
        mkdir_on_path(td.path().join("a/b/c"));
        mkdir_on_path(td.path().join("y/z"));
        write_file(td.path().join("a/b/.hide.txt"), "");
        write_file(td.path().join("a/b/ack.js"), "");
        write_file(td.path().join("a/b/foo.txt"), "");
        write_file(td.path().join("a/b/zoo.txt"), "");
        write_file(td.path().join("a/b/zoo.py"), "");
        write_file(td.path().join("a/b/zip.py"), "");
        write_file(td.path().join("y/z/foo.md"), "");

        let args = mk_args(td.path(), None, true, false, true, false);

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
}
