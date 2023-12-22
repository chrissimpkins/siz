//! This module provides support for default type definitions that are used to filter paths
//! with the ignore crate.
use std::collections::HashSet;

use crate::fuzzy::levenshtein_similarity_ratio;
use crate::types_default::DEFAULT_TYPES;

use anyhow::Result;
use colored::Colorize;
use ignore::types::{Types, TypesBuilder};
use ignore::Error;

/// A builder for creating the `ignore::types::Types` struct that is used
/// to filter files based on default path glob patterns.
///
/// The `SizTypesBuilder` struct provides methods for loading the default types,
/// defining type filters based on user input, and generating printable string
/// representations of the type names and glob patterns. It also supports approximate
/// string matching for type name suggestions when an unsupported type value is requested.
///
/// # Examples
///
/// ```
/// use siz::types::SizTypesBuilder;
///
/// let mut builder = SizTypesBuilder::new();
///
/// // Define active `ignore::types::Types` filters
/// let types = builder.filter_types(&vec![String::from("rust"), String::from("py")]);
/// ```
pub struct SizTypesBuilder {
    builder: TypesBuilder,
}

impl Default for SizTypesBuilder {
    /// Creates a new, default `SizTypesBuilder` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use siz::types::SizTypesBuilder;
    ///
    /// let builder = SizTypesBuilder::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl SizTypesBuilder {
    /// Creates a new `SizTypesBuilder` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use siz::types::SizTypesBuilder;
    ///
    /// let builder = SizTypesBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            builder: TypesBuilder::new(),
        }
    }

    fn add_type_defaults(&mut self) {
        for &(names, exts) in DEFAULT_TYPES {
            for name in names {
                for ext in exts {
                    self.builder.add(name, ext).expect("should never fail");
                }
            }
        }
    }

    fn get_approximate_match_types(&mut self, needle: &String) -> Vec<Vec<String>> {
        // the similarity ratio threshold for approximate string matching
        // this is the minimum ratio that a string must have to be considered
        // an approximate match.  The ratio is calculated using the Levenshtein
        // distance algorithm.
        let similarity_ratio_threshold = 0.75;
        let mut matches = HashSet::new();
        let mut lev_ratios: Vec<(f64, String)> = vec![];
        for ty in DEFAULT_TYPES.iter() {
            let ty_names = ty.0;
            let ty_exts = ty.1;
            let mut temp_ratios: Vec<f64> = vec![];
            // extension string matches
            for ext in ty_exts {
                let fmt_ext: &str;
                // user might have entered an extension string instead of the type name,
                // remove the "*." prefix and attempt exact match
                if ext.starts_with("*.") {
                    fmt_ext = ext.strip_prefix("*.").unwrap();
                } else if ext.starts_with('.') {
                    fmt_ext = ext.strip_prefix('.').unwrap();
                } else {
                    fmt_ext = ext;
                }
                if needle == fmt_ext {
                    // add matches to the matches HashSet, de-dupes
                    matches.insert(ty_names[0].to_string());
                } else {
                    // calculate the similarity ratio between the needle and the extension string
                    // add approximate matches to the temp_ratios vector
                    temp_ratios.push(levenshtein_similarity_ratio(needle, fmt_ext));
                }
            }
            // type name string matches
            // calculate the similarity ratio between the needle and the type name
            // and add approximate matches to the temp_ratios vector
            for name in ty_names {
                temp_ratios.push(levenshtein_similarity_ratio(needle, name));
            }

            // remove NaN values from the temp_ratios vector before the sort
            temp_ratios.retain(|f| !f.is_nan());
            // sort high to low
            temp_ratios.sort_by(|a, b| b.partial_cmp(a).unwrap());
            // keep the max similarity ratio associated with the type name
            lev_ratios.push((temp_ratios[0], ty_names[0].to_string()));
        }
        // sort the full set of type names by similarity ratio, high to low
        lev_ratios.sort_by(|a, b| b.partial_cmp(a).unwrap());
        // keep the ratios that are above the threshold value defined above, exact glob pattern string
        // matches are already in the matches HashSet.
        lev_ratios.retain(|(a, _b)| *a > similarity_ratio_threshold);

        // TODO: remove me or convert to logging
        // println!("{:?}", lev_ratios);
        let approx_matches = lev_ratios
            .iter()
            .map(|(_, b)| b.to_string())
            .collect::<Vec<String>>();

        vec![Vec::from_iter(matches), approx_matches]
    }

    /// Defines the active filter types by string input.
    ///
    /// This method takes a vector of type names and returns a `Result` containing
    /// an `ignore::types::Types` struct defined with the type name data. If an
    /// unsupported type is requested, the method performs approximate string matching
    /// to suggest alternative types.
    ///
    /// # Arguments
    ///
    /// * `types` - A vector of type names to use in filter definitions.
    ///
    /// # Returns
    ///
    /// A `Result` containing `ignore::types::Types`.
    ///
    /// # Errors
    ///
    /// This method returns an error if an unsupported type is requested.
    ///
    /// # Examples
    ///
    /// ```
    /// use siz::types::SizTypesBuilder;
    ///
    /// let mut builder = SizTypesBuilder::new();
    ///
    /// let types = builder.filter_types(&vec![String::from("rust"), String::from("py")]);
    ///
    /// match types {
    ///     Ok(filtered_types) => {
    ///        // Use the `ignore::types::Types` struct to define type name filters
    ///        // in an `ignore::WalkBuilder` instance.
    ///     },
    ///     Err(err) => {
    ///         // Handle the error
    ///     }
    /// }
    /// ```
    pub fn filter_types(&mut self, types: &Vec<String>) -> Result<Types> {
        self.add_type_defaults();
        for t in types {
            self.builder.select(t);
        }
        match self.builder.build() {
            Ok(types) => Ok(types),
            Err(err) => match err {
                Error::UnrecognizedFileType(ref name) => {
                    // user requested a type that is not supported
                    // let's approximate string match on requested type name for
                    // suggestions to return to the user.  This runs matches against
                    // path glob strings with and without the "*." prefix and approximate
                    // matches against the type names and path glob strings without the "*" or "."
                    // chars using the Levenshtein distance algorithm.  Then we format the
                    // user output with the best matches.
                    let suggestions = self.get_approximate_match_types(name);
                    let matches = &suggestions[0];
                    let matches_string = format!(
                        "Did you mean: {}? The '{}' string matched in the path glob pattern list.",
                        matches.join(" or "),
                        name
                    );
                    let approx_matches = &suggestions[1];
                    let approx_matches_string = format!(
                        "Types with approximate type name or path glob pattern string matches: {}",
                        approx_matches.join(", "),
                    );
                    let use_list_types_string =
                        "See --list-types for a list of supported type names and associated path glob patterns.";

                    let user_string: String;
                    if !matches.is_empty() {
                        user_string =
                            format!("{}\n\n{}\n\n{}", err, matches_string, use_list_types_string);
                    } else if !approx_matches.is_empty() {
                        user_string = format!(
                            "{}\n\n{}\n\n{}",
                            err, approx_matches_string, use_list_types_string
                        );
                    } else {
                        user_string = format!("{}\n\n{}", err, use_list_types_string);
                    }

                    anyhow::bail!("{}", user_string);
                }
                _ => {
                    anyhow::bail!("error building types: {}", err);
                }
            },
        }
    }
}

/// Generates a printable representation of the default type names and glob patterns.
///
/// This method returns a string containing the type names and associated glob patterns.
///
/// # Arguments
///
/// * `color` - A boolean indicating whether to include ANSI color formatting in the
/// output string.
///
/// # Returns
///
/// A string containing the printable representation of the types.
///
/// # Examples
///
/// ```
/// use siz::types::get_printable_types;
///
/// let printable_types = get_printable_types(true);
/// println!("{}", printable_types);
/// ```
pub fn get_printable_types(color: bool) -> String {
    let mut types_string = String::new();
    for &(names, exts) in DEFAULT_TYPES {
        for name in names {
            if color {
                types_string += &format!("{}:", name.blue().bold());
            } else {
                types_string += &format!("{}:", name);
            }
            for ext in exts {
                types_string += &format!(" {}", ext);
            }
            types_string += "\n";
        }
    }
    if types_string.ends_with('\n') {
        types_string = types_string.strip_suffix('\n').unwrap().to_string();
    }
    if types_string.ends_with('\r') {
        types_string = types_string.strip_suffix('\r').unwrap().to_string();
    }
    types_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_types_single() -> Result<()> {
        let mut stb = SizTypesBuilder::new();

        assert!(stb.builder.definitions().len() == 0);

        let matcher = stb.filter_types(&vec![String::from("rust")])?;

        // requested types are whitelisted
        assert!(matcher.matched("foo.rs", false).is_whitelist());
        assert!(matcher.matched("foo.py", false).is_ignore());
        // and non-requested types are ignored
        assert!(matcher.matched("foo", false).is_ignore());
        Ok(())
    }

    #[test]
    fn test_filter_types_multiple() -> Result<()> {
        let mut stb = SizTypesBuilder::new();

        assert!(stb.builder.definitions().len() == 0);

        let matcher = stb.filter_types(&vec![String::from("rust"), String::from("py")])?;

        // requested types are whitelisted
        assert!(matcher.matched("foo.rs", false).is_whitelist());
        assert!(matcher.matched("foo.py", false).is_whitelist());
        // and non-requested types are ignored
        assert!(matcher.matched("foo", false).is_ignore());
        Ok(())
    }

    #[test]
    fn test_filter_types_missing_type() -> Result<()> {
        let mut stb = SizTypesBuilder::new();

        assert!(stb.builder.definitions().len() == 0);

        // unsupported types raise an error
        assert!(stb.filter_types(&vec![String::from("bogus")]).is_err());
        // including when chained with types that are supported
        assert!(stb
            .filter_types(&vec![String::from("rust"), String::from("bogus")])
            .is_err());

        Ok(())
    }

    #[test]
    fn test_get_approximate_match_types_exact_match() {
        let mut stb = SizTypesBuilder::new();
        stb.add_type_defaults();
        let _ = stb.filter_types(&vec![String::from("jinja2")]);

        let result = stb.get_approximate_match_types(&String::from("jinja2"));

        // Assert that the result contains the expected approximate matches
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec!["jinja".to_string()]);
        assert_eq!(result[1], vec!["jinja".to_string()]);
    }

    #[test]
    fn test_get_approximate_match_types_approximate_match() {
        let mut stb = SizTypesBuilder::new();
        stb.add_type_defaults();
        let _ = stb.filter_types(&vec![String::from("xmls")]);

        let result = stb.get_approximate_match_types(&String::from("xmls"));

        // Assert that the result contains the expected approximate matches
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Vec::<String>::new());
        assert_eq!(result[1], vec!["xml".to_string(), "xls".to_string()]);
    }

    #[test]
    fn test_get_printable_types() {
        let _ = get_printable_types(false);
        let _ = get_printable_types(true);
    }
}
