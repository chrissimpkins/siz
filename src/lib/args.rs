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
    #[arg(short, long, default_value_t = false, conflicts_with = "metric_units")]
    pub binary_units: bool,

    /// ANSI colored output
    #[arg(short, long, default_value_t = false)]
    pub color: bool,

    /// Show hidden dot files and dot directories
    // Note: the logic here is reverse that used in the directory
    // walker builder.  So, we'll not this boolean value in the
    // builder instantiation block below.
    #[arg(short = 'd', long, default_value_t = false)]
    pub hidden: bool,

    /// Sort by largest to smallest file size
    #[arg(
        short = 'l',
        long,
        default_value_t = false,
        conflicts_with = "parallel",
        conflicts_with = "name"
    )]
    pub highlow: bool,

    /// Size in human readable SI metric units (powers of 1000)
    #[arg(short, long, default_value_t = false, conflicts_with = "binary_units")]
    pub metric_units: bool,

    /// Sort by filepath name
    #[arg(
        short,
        long,
        default_value_t = false,
        conflicts_with = "highlow",
        conflicts_with = "parallel"
    )]
    pub name: bool,

    /// Non-deterministic parallel recursive directory walk
    #[arg(
        short,
        long,
        default_value_t = false,
        conflicts_with = "highlow",
        conflicts_with = "name"
    )]
    pub parallel: bool,
}
