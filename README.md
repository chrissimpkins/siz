# siz

![Crates.io](https://img.shields.io/crates/v/siz)
[![Unit Tests, Stable Toolchain](https://github.com/chrissimpkins/siz/actions/workflows/stable-unittests.yml/badge.svg)](https://github.com/chrissimpkins/siz/actions/workflows/stable-unittests.yml)
[![Unit Tests, Beta Toolchain](https://github.com/chrissimpkins/siz/actions/workflows/beta-unittests.yml/badge.svg)](https://github.com/chrissimpkins/siz/actions/workflows/beta-unittests.yml)

`siz` is a customizable Rust **command line file size reporting executable** with default recursive file system traversal. It supports a number of optional path filters and sorting features, default smallest-to-largest file size sorting, command line- or .gitignore file-defined glob pattern includes/excludes, human-readable SI metric or binary block size output, and an opinionated set of default path filters.

`siz` is built with cross-platform compatibility in mind.  Project tests run in the latest macOS, Windows, and Ubuntu Linux GitHub Actions runner environments. The goal is versatile, fast, and multi-platform file size reporting.

## Contents

- [Features](#features)
  - [Default](#default)
  - [Optional](#optional)
- [Quickstart](#quickstart)
- [Installation](#installation)
  - [Minimum supported Rust version (MSRV)](#minimum-supported-rust-version-msrv)
  - [Cargo install (crates.io)](#cargo-install-cratesio)
  - [Cargo install (source repository)](#cargo-install-source-repository)
- [Usage](#usage)
  - [Default file filtering behavior](#default-file-filtering-behavior)
- [Changes](#changes)
- [Issue Reporting](#issue-reporting)
- [Contributing](#contributing)
- [License](#license)
- [Code of Conduct](#code-of-conduct)
- [Acknowledgments](#acknowledgments)

## Features

### Default

- Efficient recursive file system traversal with the `ignore` library
- File size report in bytes
- Tab-delimited report writes
- Sort by smallest-to-largest file size across the entire traversal tree
- git integration that respects .gitignore configuration ignored path definitions (in git VC repositories only)
- Exclude hidden dotfiles and recursive dot directory contents

### Optional

- Include or exclude files by .gitignore syntax glob patterns on the command line
- Include files by file type name alias. Uses an [expanded](https://github.com/chrissimpkins/siz/commits/main/src/lib/types_default.rs) ignore library type name list with additional commonly used binary file types. ([complete list source](https://github.com/chrissimpkins/siz/blob/main/src/lib/types_default.rs))
- Sort by largest-to-smallest file size
- Sort lexicographically by path names
- Define maximum directory depth traversal
- Format file sizes in human-readable SI metric block sizes (e.g., 10 MB)
- Format file sizes in human-readable binary block sizes (e.g., 10 MiB)
- Include hidden dotfiles and dot directory contents
- ANSI color support to distinguish directory parent paths from file paths
- Parallel directory traversal if execution speed is essential and file size order is not

## Quickstart

| Task | Command |
|------|---------|
| Recursive traversal, byte size sorted small to large   | `siz [DIR PATH]` |
| Filter output by .gitignore glob pattern syntax (multi-pattern support)       | `siz -g '[PATTERN_1],[PATTERN_2],...' [DIR PATH]` |
| Filter output by file type alias name (multi-alias name support)               | `siz -t '[TYPE_1],[TYPE_2],...' [DIR PATH]` |
| Sort largest to smallest size                          | `siz -l [DIR PATH]`  |
| Sort lexicographically by file path names                 | `siz -n [DIR PATH]`  |
| Set maximum directory depth traversal                  | `siz -d 2 [DIR PATH]` |
| Show hidden dotfiles and dot directory contents        | `siz -H [DIR PATH]` |
| Display human-readable file sizes in SI metric block format  | `siz -m [DIR PATH]` |
| Display human-readable file sizes in binary block format  | `siz -m [DIR PATH]` |
| ANSI coloring of directory vs. file paths              | `siz -c [DIR PATH]`   |

See `siz --help` for the list of available options.

See `siz --list-types` for the list of supported built-in type name aliases.

## Installation

### Minimum supported Rust version (MSRV)

The minimum supported Rust version is 1.70.0.

### Cargo install (crates.io)

[Install a Rust toolchain on your system](https://www.rust-lang.org/tools/install), and then use the cargo package manager to install the `siz` executable from the crates.io distribution with:

```text
cargo install siz
```

Use the same command to install updated crates.io release versions.

### Cargo install (source repository)

[Install a Rust toolchain on your system](https://www.rust-lang.org/tools/install), and then use the cargo package manager to install the `siz` executable from the main branch of this source repository with:

```text
cargo install --git https://github.com/chrissimpkins/siz.git
```

Use the `--force` option to force the installation of an updated development version over a previous installation.

## Usage

The installation provides a new `siz` executable on your system PATH.  `siz` accepts optional arguments and a single required file or directory path positional argument. Define the path at the end of your command. Data are in bytes by default. For directory traversals, the data are in smallest-to-largest size order by default. Several command line options are available to configure file size reports, including filtering, sorting, human-readable file size formatting, and ANSI color-coding paths. Please refer to the `siz --help` documentation for additional details on the options available in your commands.

### Default file filtering behavior

- Ignore all dotfiles (option available to toggle as of v0.1.0)
- Ignore all recursive traversal paths under dot directories (option available to toggle as of v0.1.0)
- Use .gitignore file glob pattern definitions to filter output *when executed on a git repository path*. By default, the tool respects .gitignore files in the parent directories of each file path.
- Respect a local .ignore file. This file supports the same glob syntax as the .gitignore file. It allows you to define different sets of includes/excludes than those defined in .gitignore, and use this file-based ignore syntax outside of a git repository.
- Will not follow symbolic links (option available to toggle as of v0.4.0)

## Changes

Please see [CHANGELOG.md](CHANGELOG.md) for changes across release versions.

## Issue Reporting

Please use the GitHub issue tracker to report bugs or other issues. Thanks!

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[Apache License, v2.0](LICENSE), unless otherwise specified.

## Code of Conduct

Please be kind and constructive.  Review our [Code of Conduct](CODE_OF_CONDUCT.md) before interacting on this repository.

## Acknowledgments

- The [ignore crate](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore): This fantastic library powers our recursive file system traversal. It's a robust and efficient solution that has greatly simplified `siz` development. A huge thanks to Andrew Gallant ([@BurntSushi](https://github.com/BurntSushi)) and all contributors to the `ignore` project for their excellent work!
