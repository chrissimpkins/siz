use std::{io::Write, path::Path, path::PathBuf, process::ExitCode};

use clap::Parser;
use humansize::{make_format, BINARY, DECIMAL};
use ignore::WalkBuilder;
use rayon::prelude::*;

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

/// Command line options
#[derive(Parser, Debug)]
#[command(name = "siz")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File or directory path
    path: PathBuf,

    /// Size in human readable binary units (powers of 1024)
    #[arg(short, long, default_value_t = false, conflicts_with = "metric_units")]
    binary_units: bool,

    /// Show hidden dot files and dot directories
    // Note: the logic here is reverse that used in the directory
    // walker builder.  So, we'll not this boolean value in the
    // builder instantiation block below.
    #[arg(short = 'd', long, default_value_t = false)]
    hidden: bool,

    /// Sort by largest to smallest file size
    #[arg(
        short = 'l',
        long,
        default_value_t = false,
        conflicts_with = "parallel",
        conflicts_with = "name"
    )]
    highlow: bool,

    /// Size in human readable SI metric units (powers of 1000)
    #[arg(short, long, default_value_t = false, conflicts_with = "binary_units")]
    metric_units: bool,

    /// Sort by filepath name
    #[arg(
        short,
        long,
        default_value_t = false,
        conflicts_with = "highlow",
        conflicts_with = "parallel"
    )]
    name: bool,

    /// Non-deterministic parallel recursive directory walk
    #[arg(
        short,
        long,
        default_value_t = false,
        conflicts_with = "highlow",
        conflicts_with = "name"
    )]
    parallel: bool,
}

fn run() -> anyhow::Result<ExitCode> {
    let args = Args::parse();

    // configure the directory walker (ignore::WalkerBuilder)
    let mut binding = WalkBuilder::new(&args.path);
    let walker_builder = binding.hidden(!args.hidden).skip_stdout(true);

    if args.parallel {
        walker_builder.build_parallel().run(|| {
            Box::new(|entry| match entry {
                Ok(entry) => match entry.metadata() {
                    Ok(metadata) => match print_file(&args, &metadata.len(), entry.path()) {
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
            print_file(&args, &path_entry.metadata()?.len(), &path_entry.path())?;
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
            print_file(&args, filesize, filepath)?;
        }
    }

    // return zero exit status code if we did not encounter an error
    Ok(ExitCode::from(0))
}

fn print_file(args: &Args, filesize: &u64, filepath: &Path) -> Result<(), std::io::Error> {
    if filepath.is_file() {
        // instantiate the human readable size formatters (humansize lib)
        let metric_size_formatter = make_format(DECIMAL);
        let binary_size_formatter = make_format(BINARY);

        if args.metric_units {
            writeln!(
                std::io::stdout(),
                "{:>9}\t{}",
                metric_size_formatter(*filesize),
                filepath.display()
            )?;
        } else if args.binary_units {
            writeln!(
                std::io::stdout(),
                "{:>10}\t{}",
                binary_size_formatter(*filesize),
                filepath.display()
            )?;
        } else {
            writeln!(std::io::stdout(), "{}\t{}", filesize, filepath.display())?;
        }
    }
    Ok(())
}
