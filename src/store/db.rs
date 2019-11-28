use std::env;
use crate::errors::{Error, Result};
use crate::indexer::TargetRange;
use crate::reader::Offset;
use crate::store::Store;

#[macro_use]
use dotenv::dotenv;

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

}

pub struct DatabaseStore {
    db: String,
}

impl DatabaseStore {
    pub fn new(db: &str) -> Result<Self> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

        Ok(
            DatabaseStore {
                db
            }
        )
    }
}

impl Store for DatabaseStore {
//    fn save(&mut self, target_name: &str, range: &TargetRange) -> Result<()> {
//
//        let row = Row {
//            target_name: target_name.to_string(),
//            coffset_start: range.file_start.coffset,
//            coffest_end: range.file_end.coffset,
//            seq_start: range.seq_start,
//            seq_end: range.seq_end,
//        };
//
//        let data = serde_json::to_string(&row)
//            .map_err(|source| Error::StoreSave { source: Box::new(source) })?;
//
//        self.file.write_all(data.as_bytes())
//            .map_err(|source| Error::StoreSave { source: Box::new(source) })?;
//
//        self.file.write_all(b"\n")
//            .map_err(|source| Error::StoreSave { source: Box::new(source) })?;
//
//        Ok(())
//    }
}
