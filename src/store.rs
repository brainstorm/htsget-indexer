use std::fs::File;
use std::io::prelude::*;

use crate::errors::{Error, Result};
use crate::indexer::TargetRange;

pub struct CsvStore {
    file: File,
}

impl CsvStore {
    pub fn new(path: &str) -> Result<Self> {
        let mut file = File::create(path)
            .map_err(|source| Error::StoreOpen { source })?;

        file.write_all(b"target_name,uoffset_start,uoffset_end,coffset_start,coffset_end,seq_start,seq_end\n");

        Ok(
            CsvStore {
                file
            }
        )
    }

    pub fn store(&mut self, target_name: &str, range: &TargetRange) -> Result<()> {
        let data = format!("{},{},{},{},{},{},{}\n",
                           target_name,
                           range.file_start.coffset, range.file_end.coffset,
                           range.file_start.uoffset, range.file_end.uoffset,
                           range.seq_start, range.seq_end);

        self.file.write_all(data.as_bytes())
            .map_err(|source| Error::StoreWrite { source })?;

        Ok(())
    }
}