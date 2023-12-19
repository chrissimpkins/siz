use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Command line options
#[derive(Parser, Debug)]
#[command(name = "siz")]
#[command(author, version, about, long_about = None, override_usage = "siz [COMMAND] [OPTIONS] PATH")]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

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
    /// Use the list-types command to view the default file types.
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

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List the default file type names and extensions used with the --type option
    ListTypes,
}
