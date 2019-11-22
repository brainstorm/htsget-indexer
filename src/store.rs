use std::fs::File;
use std::io::prelude::*;

use crate::errors::{Error, Result};
use crate::reader::BamRead;

pub struct TsvStore {
    file: File,
}

impl TsvStore {
    pub fn new(path: &str) -> Result<Self> {
        let file = File::create(path)
            .map_err(|source| Error::StoreOpen { source })?;
        Ok(
            TsvStore {
                file
            }
        )
    }

    pub fn store(&mut self, read: &BamRead) -> Result<()> {
        let data = format!("{}\t{}\t{}\t{}",
                           read.voffset.coffset, read.voffset.uoffset,
                           read.start, read.end);

        self.file.write_all(data.as_bytes())
            .map_err(|source| Error::StoreWrite { source })?;

        Ok(())
    }
}