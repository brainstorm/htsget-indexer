use crate::reader::BamReader;
use crate::errors::Result;
use crate::store::TsvStore;

pub struct BlockIndexer {
    reader: BamReader,
    store: TsvStore,
    bsize: usize,
}

impl BlockIndexer {
    pub fn new(reader: BamReader, store: TsvStore, bsize: usize) -> Self {
        BlockIndexer {
            reader,
            store,
            bsize
        }
    }

    pub fn run(&mut self) -> Result<()> {
        Ok(())
    }
}