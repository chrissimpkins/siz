//! This crate provides functionality for the siz command line executable file system utility.
//!
//! It includes the following modules:
//!
//! * `args`: This module provides structures and functions for parsing command line arguments.
//! * `format`: This module provides functions for formatting the output.
//! * `fuzzy`: This module provides functions for approximate string matching of file types.
//! * `stdstreams`: This module provides functions for working with standard input and output streams.
//! * `types`: This module provides type definitions used throughout the crate.
//! * `types_default`: This module provides default type name and associated path glob pattern definitions.
//! * `walk`: This module provides support for walking the file system.

pub mod args;
pub mod format;
pub mod fuzzy;
pub mod stdstreams;
pub mod types;
pub mod types_default;
pub mod walk;
