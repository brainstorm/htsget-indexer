use crate::reader::BamReader;
use crate::indexer::BlockIndexer;

pub mod reader;
pub mod indexer;
pub mod store;
pub mod errors;

use errors::Result;

#[cfg(feature = "csv")]
use crate::store::csv::CsvStore;

#[cfg(feature = "json")]
use crate::store::json::JsonStore;

#[cfg(feature = "database")]
use crate::store::database::DatabaseStore;


fn main() -> Result<()> {
    let reader_path = "tests/data/htsnexus_test_NA12878.bam".to_string();
    let reader = BamReader::new(reader_path)?;

    let store = create_store()?;

    let block_size: usize = 64 * 1024;
    let mut indexer = BlockIndexer::new(reader, store, block_size);

    indexer.run()
}

#[cfg(feature = "csv")]
fn create_store() -> Result<CsvStore> {
    CsvStore::new("index.csv")
}

#[cfg(feature = "json")]
fn create_store() -> Result<JsonStore> {
    JsonStore::new("index.json")
}

#[cfg(feature = "database")]
fn create_store() -> Result<DatabaseStore> {
    DatabaseStore::new("postgres://chris:mola@localhost:54320/htsget")
}