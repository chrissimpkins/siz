//! `siz` is a command line executable file system utility for file size reporting.
//! This crate provides the core functionality for the executable.  It is not
//! designed for use as a third party library.

pub mod args;
pub mod format;
pub mod fuzzy;
pub mod stdstreams;
pub mod types;
pub mod types_default;
pub mod walk;
