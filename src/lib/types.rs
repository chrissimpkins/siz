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
