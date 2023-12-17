# siz

[![Unit Tests, Stable Toolchain](https://github.com/chrissimpkins/siz/actions/workflows/stable-unittests.yml/badge.svg)](https://github.com/chrissimpkins/siz/actions/workflows/stable-unittests.yml)
[![Unit Tests, Beta Toolchain](https://github.com/chrissimpkins/siz/actions/workflows/beta-unittests.yml/badge.svg)](https://github.com/chrissimpkins/siz/actions/workflows/beta-unittests.yml)
![Crates.io](https://img.shields.io/crates/v/siz)

`siz` is a customizable Rust **command line file size reporting executable** with support for recursive traversal of file systems. It supports a number of optional filters and sorting features, default smallest-to-largest file size sorting, command line- or .gitignore file-defined glob pattern includes/excludes, human-readable SI metric or binary block size output, and an opinionated set of default (but toggleable) path filters.

Project tests run in the latest macOS, Windows, and Ubuntu Linux GitHub Actions runner environments. Broad platform support is a project goal.

## Contents

- [Features](#features)
- [Quickstart](#quickstart)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

## Features

### Default

- Efficient recursive file system traversal with the `ignore` library
- File size report in bytes
- Sort by smallest-to-largest file size across the entire traversal tree
- Filter paths with .gitignore definitions (in git VC repositories only)
- Exclude hidden dotfiles and recursive dot directory contents

### Optional

- Include or exclude files by .gitignore syntax glob patterns on the command line
- Sort by largest-to-smallest file size
- Sort lexicographically by path names
- Define maximum directory depth traversal
- Human readable SI metric (powers of 1000) block sizes
- Human readable binary (powers of 1024) block sizes
- Include hidden dotfiles and dot directory contents
- ANSI color support to distinguish directory paths from file paths
- Parallel directory traversal if execution speed is essential and output order is not

## Quickstart

| Task | Command |
|------|---------|
| Recursive traversal, byte size sorted small to large   | `siz [DIR PATH]` |
| Filter output by .gitignore glob pattern syntax        | `siz -g '[PATTERN_1],[PATTERN_2],...' [DIR PATH]` |
| Sort largest to smallest size                          | `siz -l [DIR PATH]`  |
| Sort lexicographically by file path names                 | `siz -n [DIR PATH]`  |
| Set maximum directory depth traversal                  | `siz -d 2 [DIR PATH]` |
| Show hidden dotfiles and dot directory contents        | `siz -H [DIR PATH]` |
| Display human-readable file sizes in SI metric block format  | `siz -m [DIR PATH]` |
| Display human-readable file sizes in binary block format  | `siz -m [DIR PATH]` |
| ANSI coloring of directory vs. file paths              | `siz -c [DIR PATH]`   |

See `siz --help` for the list of available options.

## Installation

### Minimum supported Rust version (MSRV)

The minimum supported Rust version is 1.70.0.

### Cargo install (crates.io)

Install a Rust toolchain on your system, and then use the cargo package manager to install the `siz` executable from the crates.io distribution with:

```
$ cargo install siz
```

### Cargo install (source repository)

Install a Rust toolchain on your system, and then use the cargo package manager to install the `siz` executable from the main branch of this source repository with:

```
cargo install --git https://github.com/chrissimpkins/siz.git
```

## Usage

The installation provides a new `siz` executable on your system PATH.  `siz` accepts optional arguments and a single required file or directory path positional argument. Define the path at the end of your command. Data are in bytes by default. For directory traversals, the data are in smallest to largest size order by default. Several command line options are available to configure file size reports, including filtering, sorting, human-readable file size formatting, and ANSI color-coding paths. Please refer to the `siz --help` documentation for additional details on the options available in your commands.

Notable default file filter behaviors include:

- Ignore all dotfiles (toggleable as of v0.1.0)
- Ignore all recursive traversal paths under dot directories (toggleable as of v0.1.0)
- Use .gitignore file glob pattern definitions to filter output *when executed in a git repository*. By default, the tool respects .gitignore files in the parent directories of each file path.
- Respect a local .ignore file. This file supports the same glob syntax as the .gitignore file and allows you to define different sets of includes/excludes than those defined in .gitignore
- Will not follow symbolic links

## Changes

Please see [CHANGELOG.md](CHANGELOG.md).

## License

[Apache License, v2.0](LICENSE)