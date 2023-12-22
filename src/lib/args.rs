use std::path::PathBuf;

use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Red.on_default() | Effects::BOLD)
        .usage(AnsiColor::Red.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

/// `Args` is a struct that represents the command line arguments passed to the program.
///
/// It uses the `clap::Parser` derive macro to parse the command line arguments.
///
/// # Fields
///
/// * `path`: An optional `PathBuf` that represents the file or directory path.
///
/// * `binary_units`: A boolean that indicates whether to display the size in human readable binary units (powers of 1024).
///
/// * `color`: A boolean that indicates whether to use ANSI colored output.
///
/// * `depth`: An optional `usize` that represents the maximum directory traversal depth.
///
/// * `glob`: An optional vector of `String` that represents path glob patterns to filter the output.
///
/// * `hidden`: A boolean that indicates whether to show hidden dot files and dot directories.
///
/// * `highlow`: A boolean that indicates whether to sort the output by largest to smallest file size.
///
/// * `list-types`: A boolean that indicates whether to print the built-in types available for use with the types filter.
///
/// * `metric_units`: A boolean that indicates whether to display the size in human readable SI metric units (powers of 1000).
///
/// * `name`: A boolean that indicates whether to sort the output by path name.
///
/// * `parallel`: A boolean that indicates whether to use parallel recursive directory walk (non-deterministic order).
///
/// * `default_type`: An optional vector of `String` that represents file type names to filter the output.
#[derive(Parser, Debug)]
#[command(name = "siz")]
#[command(author, version, about, styles = styles(), long_about = None, override_usage = "siz [COMMAND] [OPTIONS] PATH")]
pub struct Args {
    /// File or directory path
    pub path: Option<PathBuf>,

    /// Size in human readable binary units (powers of 1024)
    #[arg(
        short,
        long,
        default_value_t = false,
        conflicts_with = "metric_units",
        help_heading = "Size Formats"
    )]
    pub binary_units: bool,

    /// ANSI colored output
    #[arg(short, long, default_value_t = false, help_heading = "Color")]
    pub color: bool,

    /// Maximum directory traversal depth
    #[arg(short, long, help_heading = "Filters")]
    pub depth: Option<usize>,

    /// Filter the output by gitignore syntax glob patterns
    #[arg(
        short,
        long,
        value_delimiter = ',',
        conflicts_with = "default_type",
        help_heading = "Filters"
    )]
    pub glob: Option<Vec<String>>,

    /// Show hidden dot files and dot directories
    // Note: the logic here is reverse that used in the directory
    // walker builder.  So, we'll not this boolean value in
    // execution logic.
    #[arg(short = 'H', long, default_value_t = false, help_heading = "Filters")]
    pub hidden: bool,

    /// Sort by largest to smallest file size
    #[arg(
        short = 'l',
        long,
        default_value_t = false,
        conflicts_with = "parallel",
        conflicts_with = "name",
        help_heading = "Sorting"
    )]
    pub highlow: bool,

    /// Print the built-in types available for use with the types filter
    #[arg(long = "list-types", default_value_t = false, help_heading = "Filters")]
    pub list_types: bool,

    /// Size in human readable SI metric units (powers of 1000)
    #[arg(
        short,
        long,
        default_value_t = false,
        conflicts_with = "binary_units",
        help_heading = "Size Formats"
    )]
    pub metric_units: bool,

    /// Sort by path name
    #[arg(
        short,
        long,
        default_value_t = false,
        conflicts_with = "highlow",
        conflicts_with = "parallel",
        help_heading = "Sorting"
    )]
    pub name: bool,

    /// Parallel recursive directory walk (non-deterministic order)
    #[arg(
        short,
        long,
        default_value_t = false,
        conflicts_with = "highlow",
        conflicts_with = "name",
        help_heading = "Sorting"
    )]
    pub parallel: bool,

    /// Filter the output by one or more comma separated file type names.
    /// Use the list-types option to view a list of the built-in file types.
    #[arg(
        short = 't',
        long = "type",
        value_delimiter = ',',
        value_names = ["TY1,TY2,..."],
        conflicts_with = "glob",
        help_heading = "Filters"
    )]
    pub default_type: Option<Vec<String>>,
}
