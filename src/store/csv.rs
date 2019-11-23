use std::fs::File;
use std::io::prelude::*;

use crate::errors::{Error, Result};
use crate::indexer::TargetRange;
use crate::store::Store;

pub struct CsvStore {
  file: File,
}

impl CsvStore {
  pub fn new(path: &str) -> Result<Self> {
    let mut file = File::create(path)
        .map_err(|source| Error::StoreCreate { source: Box::new(source) })?;

    file.write_all(b"target,bytes_start,bytes_end,seq_start,seq_end\n")
        .map_err(|source| Error::StoreCreate { source: Box::new(source) })?;

    Ok(
      CsvStore {
        file
      }
    )
  }
}

impl Store for CsvStore {
  fn save(&mut self, target_name: &str, range: &TargetRange) -> Result<()> {
    let data = format!("{},{},{},{},{}\n",
                       target_name,
                       range.file_start.coffset, range.file_end.coffset,
                       range.seq_start, range.seq_end);

    self.file.write_all(data.as_bytes())
        .map_err(|source| Error::StoreSave { source: Box::new(source) })?;

    Ok(())
  }
}
