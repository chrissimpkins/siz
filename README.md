# siz

![Crates.io](https://img.shields.io/crates/v/siz)
[![Unit Tests, Stable Toolchain](https://github.com/chrissimpkins/siz/actions/workflows/stable-unittests.yml/badge.svg)](https://github.com/chrissimpkins/siz/actions/workflows/stable-unittests.yml)
[![Unit Tests, Beta Toolchain](https://github.com/chrissimpkins/siz/actions/workflows/beta-unittests.yml/badge.svg)](https://github.com/chrissimpkins/siz/actions/workflows/beta-unittests.yml)

`siz` is a customizable Rust **command line file size reporting executable** with support for recursive traversal of file systems. It supports a number of optional filters and sorting features, default smallest-to-largest file size sorting, command line- or .gitignore file-defined glob pattern includes/excludes, human-readable SI metric or binary block size output, and an opinionated set of default (but toggleable) path filters.

Project tests run in the latest macOS, Windows, and Ubuntu Linux GitHub Actions runner environments. Broad platform support is a project goal.

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
- [Changes](#changes)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

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
cargo install siz
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

## Contributing

We welcome contributions to the `siz` project under the Apache License v2.0. Whether you're looking to fix bugs, add new features, or improve documentation, your help is greatly appreciated. 

Here's how you can contribute:

1. **Fork the Repository**: Fork the `siz` repository to your own GitHub account.

2. **Clone the Repository**: Clone the forked repository to your local machine.

    ```text
    git clone https://github.com/<your-username>/siz.git
    ```

3. **Create a New Branch**: Create a new branch for your changes.

    ```text
    git checkout -b add-new-feature
    ```

4. **Make Your Changes**: Make your changes to the code or documentation.

5. **Commit Your Changes**: Commit your changes to your branch. Include a commit message that briefly describes your changes.

    ```text
    git commit -m "Add new feature"
    ```

6. **Push Your Changes**: Push your changes to your forked repository on GitHub.

    ```text
    git push origin add-new-feature
    ```

7. **Submit a Pull Request**: Go to the `siz` repository on GitHub, and click the "New pull request" button. Select your fork and the branch that contains your changes. Click the "Create pull request" button to submit your pull request.

Before submitting a pull request, please make sure your code compiles and all tests pass, including the clippy lints. If you're adding a new feature, please add tests that cover the new feature.

You can run the following commands in the root of your downstream source repository to execute these tests before you submit your pull request:

```text
cargo test
cargo clippy
```

If you have any questions or need help with contributing, please feel free to reach out.

## License

[Apache License, v2.0](LICENSE) unless otherwise specified.

## Acknowledgments

This project uses the fantastic [ignore crate](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore) for recursive file system traversal. This library allows us to support a set of filtered path traversal defaults, command line filtering options, and ignore configuration file support that will feel familiar to users of `ripgrep` and other projects that use this library.  A huge thanks goes out to the ignore project authors for this great resource.
