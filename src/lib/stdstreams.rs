//! This module provides functions for working with standard input and output streams.
use std::io::Write;
use std::path::{Path, MAIN_SEPARATOR_STR};
use std::sync::OnceLock;

use crate::args::Args;
use colored::*;

/// A `std::sync::OnceLock` for the platform-specific colored separator
/// string used to format file path output.
static COLORED_SEPARATOR_STR: OnceLock<ColoredString> = OnceLock::new();

/// Writes the given `filesize` and `filepath` to stdout.
///
/// # Arguments
///
/// * `filesize` - The size of the file to be written.
/// * `filepath` - The path of the file to be written.
///
/// # Returns
///
/// Returns `Ok(())` if the write operation is successful, otherwise returns an `std::io::Error`.
#[inline(always)]
pub fn write_stdout<T, U>(filesize: T, filepath: U) -> Result<(), std::io::Error>
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    writeln!(std::io::stdout(), "{}\t{}", filesize, filepath)?;
    Ok(())
}

/// Formats and prints the file information to the standard output stream based
/// on the provided arguments.
///
/// # Arguments
///
/// * `args` - The command line arguments.
/// * `filesize` - The size of the file.
/// * `filepath` - The path of the file.
/// * `metric_size_formatter` - The function to format the file size in metric units.
/// * `binary_size_formatter` - The function to format the file size in binary units.
///
/// # Returns
///
/// Returns `Ok(())` if the formatting and printing is successful, otherwise returns
/// a `std::io::Error`.
#[inline(always)]
pub fn format_print_file(
    args: &Args,
    filesize: &u64,
    filepath: &Path,
    metric_size_formatter: impl Fn(u64) -> String,
    binary_size_formatter: impl Fn(u64) -> String,
) -> Result<(), std::io::Error> {
    // exclude directories, filter on files only
    if args.color {
        let fmt_filepath = match filepath.parent() {
            Some(ppath) => match filepath.file_name() {
                Some(fpath) => {
                    format!(
                        "{}{}{}",
                        ppath.to_string_lossy().blue(),
                        COLORED_SEPARATOR_STR.get_or_init(|| MAIN_SEPARATOR_STR.blue()),
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
    Ok(())
}
