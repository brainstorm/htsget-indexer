use std::env;
use crate::errors::{Error, Result};
use crate::indexer::TargetRange;
use crate::reader::Offset;
use crate::store::Store;

#[macro_use]
use dotenv::dotenv;

use dotenv;
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;


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
            bam_id: String,
            target_name: target_name.to_string(),
            bytes_start: range.file_start.coffset,
            bytes_end: range.file_end.coffset,
            seq_start: range.seq_start,
            seq_end: range.seq_end,
        };

        // Establish connection

        // Write the row

        Ok(())
    }

    fn open(&mut self, database: &str) {
//        dotenv().ok();

//        let database_url = env::var("DATABASE_URL")
//            .expect("DATABASE_URL must be set");

        PgConnection::establish(&database)
            .expect(&format!("Error connecting to {}", database));
    }
}
