// standard library
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

// external libraries
use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use rayon::prelude::*;

// size library
use siz::args::Args;
use siz::format::{build_binary_size_formatter, build_metric_size_formatter};
use siz::stdstreams::format_print_file;
use siz::types::get_printable_types;
use siz::walk::{FileWalker, ParallelWalker};

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
            let _ = writeln!(std::io::stderr(), "{} {:#}", "Error:".red().bold(), err);
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<ExitCode> {
    let args = Args::parse();

    // Short circuit argument handling
    // The block below will return exit status codes without
    // further execution
    if args.list_types {
        let types_string = get_printable_types();
        println!("{}", types_string);
        return Ok(ExitCode::from(0));
    }

    // --------------------------------------------------------------
    // IMPORTANT: must keep the presence of a path definition check
    // here because we unwrap the Option in other places in the code.
    // --------------------------------------------------------------
    // Validation: command line path argument
    match &args.path {
        Some(path) => {
            if !path.exists() {
                anyhow::bail!("path does not exist: {}", path.display());
            }
        }
        None => {
            anyhow::bail!("a file or directory path argument is required. Enter a path at the end of your command.");
        }
    }

    // instantiate the human readable size formatters (humansize lib)
    let metric_size_formatter = build_metric_size_formatter();
    let binary_size_formatter = build_binary_size_formatter();

    if args.parallel {
        // unsorted, parallel directory walk output
        ParallelWalker::new(&args)?.print_files(
            &args,
            &metric_size_formatter,
            &binary_size_formatter,
        )?;
    } else if args.name {
        // file path name sorted output
        for entry in FileWalker::new(&args)? {
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
        // default: file size sorted output
        let mut v: Vec<(u64, PathBuf)> = Vec::with_capacity(250);
        // recursively walk the directory and fill Vec with
        // (file size, file path) data
        for entry in FileWalker::new(&args)? {
            let path_entry = entry?;
            v.push((path_entry.metadata()?.len(), path_entry.into_path()));
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
