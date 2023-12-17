use std::io::Write;
use std::path::{Path, MAIN_SEPARATOR_STR};
use std::sync::OnceLock;

use crate::args::Args;
use colored::*;

static COLORED_SEPARATOR_STR: OnceLock<ColoredString> = OnceLock::new();

#[inline(always)]
pub fn write_stdout<T, U>(filesize: T, filepath: U) -> Result<(), std::io::Error>
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    writeln!(std::io::stdout(), "{}\t{}", filesize, filepath)?;
    Ok(())
}

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
