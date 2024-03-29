use std::fs::File;
use std::io::prelude::*;
use serde::Serialize;

use crate::errors::{Error, Result};
use crate::indexer::TargetRange;
use crate::reader::Offset;
use crate::store::Store;

#[derive(Serialize)]
struct Row {
  target_name: String,
  coffset_start: Offset,
  coffest_end: Offset,
  seq_start: i32,
  seq_end: i32,
}

pub struct JsonStore {
    file: File,
}

impl JsonStore {
    pub fn new(path: &str) -> Result<Self> {
        let file = File::create(path)
            .map_err(|source| Error::StoreCreate { source: Box::new(source) })?;

        Ok(
            JsonStore {
                file
            }
        )
    }
}

impl Store for JsonStore {
  fn save(&mut self, target_name: &str, range: &TargetRange) -> Result<()> {

    let row = Row {
      target_name: target_name.to_string(),
      coffset_start: range.file_start.coffset,
      coffest_end: range.file_end.coffset,
      seq_start: range.seq_start,
      seq_end: range.seq_end,
    };

    let data = serde_json::to_string(&row)
        .map_err(|source| Error::StoreSave { source: Box::new(source) })?;

    self.file.write_all(data.as_bytes())
        .map_err(|source| Error::StoreSave { source: Box::new(source) })?;

    self.file.write_all(b"\n")
        .map_err(|source| Error::StoreSave { source: Box::new(source) })?;

    Ok(())
  }
}
