# Changelog

## v0.4.0

- new feature: add optional report of symbolic link follow file paths
- new option: add `--follow` (or `-L`) command line option
- dependency update: update clap 4.4.11 to 4.4.12

## v0.3.0

- add a custom release profile with definitions for codegen-units=1, fat LTO, and binary stripping. This significantly reduces the size of the executable binary.
- dev-dependency update: tempfile 3.8.1 to 3.9.0

## v0.2.0

- new feature: add optional built-in file type name alias filtering support with the `--type` (also `-t`) option
- new feature: add an optional standard output write of the supported file type name aliases and associated path glob patterns with the `--list-types` option. Includes optional ANSI color support when used with the `--color` (or `-c`) option.
- new options: add `--type` (or `-t`) and `--list-types` command line options
- add DEFAULT_TYPES file type alias name to file path glob pattern map source from upstream ignore project under MIT license.  The source was expanded in this release work to include many new (mostly) binary file type alias names for additional commonly used (mostly) binary file types.
- add exact glob pattern matching and Levenshtein distance-based approximate string matching recommendations for invalid user type name alias entries. The recommendations are available in the user error report following execution with an invalid `--type` file type name alias definition argument
- add column alignment in `--help` documentation content
- add default ANSI color in `--help` documentation
- expand public siz library documentation across the entire library
- dependency update: anyhow 1.0.75 to 1.0.76
- dependency update: add the clap `wrap_help` feature
- dev-dependency update: add new criterion and approx crate development dependencies
- add Levenshtein distance function benchmark test
- add Code of Conduct
- add Contributing documentation on the README
- configure GitHub Action CI test runners to use cargo color always

## v0.1.0

- initial release
