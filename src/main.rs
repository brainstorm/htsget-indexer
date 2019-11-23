use crate::reader::BamReader;
use crate::indexer::BlockIndexer;

pub mod reader;
pub mod indexer;
pub mod store;
pub mod errors;

use errors::Result;
use crate::store::CsvStore;

fn main() -> Result<()> {
    let reader_path = "tests/data/htsnexus_test_NA12878.bam".to_string();
    let reader = BamReader::new(reader_path)?;

    let store_path = "index.json";
    let store = CsvStore::new(store_path)?;

    let block_size: usize = 64 * 1024;
    let mut indexer = BlockIndexer::new(reader, store, block_size);

    indexer.run()
}
