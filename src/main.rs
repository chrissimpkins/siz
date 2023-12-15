// standard library
use std::{
    path::Path,
    path::{PathBuf, MAIN_SEPARATOR_STR},
    process::ExitCode,
};

// external libraries
use clap::Parser;
use colored::*;
use humansize::{make_format, BINARY, DECIMAL};
use ignore::WalkBuilder;
use rayon::prelude::*;

// size library
use size::args::Args;
use size::stdstreams::write_stdout;

// main entry point for the siz executable
fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            for cause in err.chain() {
                if let Some(ioerr) = cause.downcast_ref::<std::io::Error>() {
                    if ioerr.kind() == std::io::ErrorKind::BrokenPipe {
                        return ExitCode::from(0);
                    }
                }
            }
            eprintln!("{:#}", err);
            ExitCode::from(1)
        }
    }
}

fn run() -> anyhow::Result<ExitCode> {
    let args = Args::parse();

    // configure the directory walker (ignore::WalkerBuilder)
    let mut binding = WalkBuilder::new(&args.path);
    let walker_builder = binding.hidden(!args.hidden).skip_stdout(true);

    // instantiate the human readable size formatters (humansize lib)
    let metric_size_formatter = make_format(DECIMAL);
    let binary_size_formatter = make_format(BINARY);

    if args.parallel {
        walker_builder.build_parallel().run(|| {
            Box::new(|entry| match entry {
                Ok(entry) => match entry.metadata() {
                    Ok(metadata) => match format_print_file(
                        &args,
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
                                if let Some(ioerr) = cause.downcast_ref::<std::io::Error>() {
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
                },
                Err(e) => {
                    eprintln!("Error reading entry: {}", e);
                    ignore::WalkState::Quit
                }
            })
        })
    } else if args.name {
        // extend the ignore::WalkerBuilder with sort_by_file_path sorting
        let dir_walker = walker_builder.sort_by_file_path(|a, b| a.cmp(b)).build();

        for entry in dir_walker {
            let path_entry = entry?;
            format_print_file(
                &args,
                &path_entry.metadata()?.len(),
                path_entry.path(),
                &metric_size_formatter,
                &binary_size_formatter,
            )?;
        }
    } else {
        let mut v: Vec<(u64, PathBuf)> = Vec::with_capacity(250);
        // instantiate the WalkBuilder directory walker, includes filters support (ignore lib)
        let dir_walker = walker_builder.build();

        // recursively walk the directory and fill Vec with
        // (file size, file path) data
        for entry in dir_walker {
            let path_entry = entry?;
            let metadata = path_entry.metadata()?;
            if path_entry.path().is_file() {
                v.push((metadata.len(), path_entry.path().to_path_buf()));
            }
        }

        // sort the files by size in place, in parallel with rayon lib
        if args.highlow {
            // we reverse the sort by swapping the tuple compare order in this line
            v.par_sort_unstable_by(|(a, b), (c, d)| (c, d).cmp(&(a, b)));
        } else {
            v.par_sort_unstable_by(|(a, b), (c, d)| (a, b).cmp(&(c, d)));
        }

        // Print the report to stdout
        for (filesize, filepath) in v.iter() {
            format_print_file(
                &args,
                filesize,
                filepath,
                &metric_size_formatter,
                &binary_size_formatter,
            )?;
        }
    }
    // return zero exit status code if we did not encounter an error
    Ok(ExitCode::from(0))
}

fn format_print_file(
    args: &Args,
    filesize: &u64,
    filepath: &Path,
    metric_size_formatter: impl Fn(u64) -> String,
    binary_size_formatter: impl Fn(u64) -> String,
) -> Result<(), std::io::Error> {
    // exclude directories, filter on files only
    if filepath.is_file() {
        if args.color {
            let fmt_filepath = match filepath.parent() {
                Some(ppath) => match filepath.file_name() {
                    Some(fpath) => {
                        format!(
                            "{}{}{}",
                            ppath.to_string_lossy().blue(),
                            MAIN_SEPARATOR_STR.blue(),
                            fpath.to_string_lossy()
                        )
                    }
                    None => format!("{}", ppath.to_string_lossy().blue()),
                },
                None => String::from(""),
            };

            let fmt_filesize: String;
            if args.metric_units {
                fmt_filesize = format!("{:>9}", metric_size_formatter(*filesize));
                write_stdout(&fmt_filesize, &fmt_filepath)?;
            } else if args.binary_units {
                fmt_filesize = format!("{:>10}", binary_size_formatter(*filesize));
                write_stdout(&fmt_filesize, &fmt_filepath)?;
            } else {
                write_stdout(filesize, &fmt_filepath)?;
            }
        } else if args.metric_units {
            write_stdout(
                format!("{:>9}", metric_size_formatter(*filesize)),
                filepath.display(),
            )?;
        } else if args.binary_units {
            write_stdout(
                format!("{:>10}", binary_size_formatter(*filesize)),
                filepath.display(),
            )?;
        } else {
            write_stdout(filesize, filepath.display())?;
        }
    }
    Ok(())
}
