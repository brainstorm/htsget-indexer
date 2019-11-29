use std::env;
use crate::errors::{Error, Result};
use crate::indexer::TargetRange;
use crate::reader::Offset;
use crate::store::Store;

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

// XXX: Consolidate/share this struct with JsonStore's Row?
//#[derive(Queryable)]
struct Row {
    bam_id: String,
    target_name: String,
    bytes_start: Offset,
    bytes_end: Offset,
    seq_start: i32,
    seq_end: i32,
}

pub struct DatabaseStore {
    db: String,
}

impl DatabaseStore {
    pub fn new(db: String) -> Result<Self> {
        PgConnection::establish(&db)
            .unwrap_or_else(|_| panic!("Error connecting to {}", db));

        Ok(
            DatabaseStore {
                db
            }
        )
    }
}

impl Store for DatabaseStore {
    fn save(&mut self, bam_id: &str, target_name: &str, range: &TargetRange) -> Result<()> {
        let row = Row {
            bam_id: bam_id.to_string(),
            target_name: target_name.to_string(),
            bytes_start: range.file_start.coffset,
            bytes_end: range.file_end.coffset,
            seq_start: range.seq_start,
            seq_end: range.seq_end,
        };

        // Write the row
        diesel::insert_into(htsget_blocks::table)
            .values(&row);

        Ok(())
    }
}
