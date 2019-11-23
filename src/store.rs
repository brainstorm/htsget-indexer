use std::fs::File;
use std::io::prelude::*;

use crate::errors::{Error, Result};
use crate::reader::{BamRead, TargetId};
use crate::indexer::TargetRange;

pub struct TsvStore {
    file: File,
}

impl TsvStore {
    pub fn new(path: &str) -> Result<Self> {
        let mut file = File::create(path)
            .map_err(|source| Error::StoreOpen { source })?;

        file.write_all(b"target_name\tuoffset_start\tuoffset_end\tcoffset_start\tcoffset_end\tseq_start\tseq_end\n");

        Ok(
            TsvStore {
                file
            }
        )
    }

    pub fn store(&mut self, target_name: &str, range: &TargetRange) -> Result<()> {
        let data = format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                           target_name,
                           range.file_start.coffset, range.file_end.coffset,
                           range.file_start.uoffset, range.file_end.uoffset,
                           range.seq_start, range.seq_end);

        self.file.write_all(data.as_bytes())
            .map_err(|source| Error::StoreWrite { source })?;

        Ok(())
    }
}