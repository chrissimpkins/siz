use crate::types_default::DEFAULT_TYPES;

use anyhow::Result;
use ignore::types::{Types, TypesBuilder};

pub struct SizTypesBuilder {
    builder: TypesBuilder,
}

impl Default for SizTypesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SizTypesBuilder {
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

    pub fn filter_types(&mut self, types: &Vec<String>) -> Result<Types> {
        self.add_type_defaults();
        for t in types {
            self.builder.select(t);
        }
        Ok(self.builder.build()?)
    }
}

pub fn get_printable_types() -> String {
    let mut types_string = String::new();
    for &(names, exts) in DEFAULT_TYPES {
        for name in names {
            types_string += &format!("{}:", name);
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
    fn test_get_printable_types() {
        let _ = get_printable_types();
    }
}
