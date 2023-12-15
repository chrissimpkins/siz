use anyhow::{Error, Result};
use ignore::WalkBuilder;

use crate::args::Args;

pub struct Walker {
    walker: ignore::Walk,
}

impl Walker {
    pub fn new(args: &Args) -> Self {
        let mut binding = WalkBuilder::new(&args.path);
        let walker = binding.hidden(!args.hidden).skip_stdout(true);

        // sort by file path string
        if args.name {
            walker.sort_by_file_path(|a, b| a.cmp(b));
        }

        Self {
            walker: walker.build(),
        }
    }
}

impl Iterator for Walker {
    type Item = Result<ignore::DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.walker.next() {
            Some(Ok(entry)) => Some(Ok(entry)),
            Some(Err(err)) => Some(Err(Error::new(err))),
            None => None,
        }
    }
}
