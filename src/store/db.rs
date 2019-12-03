use crate::errors::{Error, Result};
use crate::indexer::TargetRange;
use crate::reader::Offset;
use crate::store::Store;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::store::schema::htsget_blocks::dsl::*;
use crate::store::models::HtsgetBlock;

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
    fn save<T, Q>(&mut self, bam_id: bam_id,
                       target_name: target_name,
                       range: &TargetRange) -> Result<()> {
        let row = HtsgetBlock {
            bam_id: bam_id,
            target_name: target_name,
            bytes_start: range.file_start.coffset as i64,
            bytes_end: range.file_end.coffset as i64,
            seq_start: range.seq_start as i64,
            seq_end: range.seq_end as i64,
        };

        // Write the row
        diesel::insert_into(htsget_blocks)
            .values(&row);

        Ok(())
    }
}
