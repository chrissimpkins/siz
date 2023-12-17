use std::path::PathBuf;

use clap::Parser;

/// Command line options
#[derive(Parser, Debug)]
#[command(name = "siz")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// File or directory path
    pub path: PathBuf,

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
    #[arg(short, long, value_delimiter = ',', help_heading = "Filters")]
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
}
