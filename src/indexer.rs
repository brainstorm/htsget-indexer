use crate::reader::{BamReader, BamRead, FileOffsets, TargetId};
use crate::errors::{ Error, Result };
use crate::store::TsvStore;
use std::collections::HashMap;
use std::ops::Range;
use std::convert::TryFrom;

pub struct TargetRange {
    pub file_start: FileOffsets,
    pub file_end: FileOffsets,
    pub seq_start: i32,
    pub seq_end: i32,
}

pub struct BlockIndexer {
    reader: BamReader,
    store: TsvStore,
    block_size: usize,
}

impl BlockIndexer {
    pub fn new(reader: BamReader, store: TsvStore, block_size: usize) -> Self {
        BlockIndexer {
            reader,
            store,
            block_size,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut maybe_last_file_offset: Option<FileOffsets> = None;
        let mut read_bytes: usize = 0;
        let mut intervals = HashMap::<TargetId, TargetRange>::new();
        let target_names = self.reader.target_names();
        let target_names_len = i32::try_from(target_names.len())
            .map_err(|source| Error::TargetNamesTooLong { source })?;

        loop {
            match self.reader.read() {
                Ok(Some(read)) => {
                    if read.target_id >= 0 && read.target_id < target_names_len {
                        let entry = intervals.entry(read.target_id)
                            .or_insert(TargetRange {
                                file_start: FileOffsets::from_offset(i64::max_value()),
                                file_end: FileOffsets::from_offset(0),
                                seq_start: i32::max_value(),
                                seq_end: i32::min_value(),
                            });

                        entry.file_start = FileOffsets::min(entry.file_start, read.file_start);
                        entry.file_end = FileOffsets::max(entry.file_end, read.file_end);
                        entry.seq_start = i32::min(entry.seq_start, read.seq_start);
                        entry.seq_end = i32::max(entry.seq_end, read.seq_end);

                        if let Some(last_file_offset) = maybe_last_file_offset {
                            read_bytes += if read.file_end.coffset == last_file_offset.coffset {
                                (read.file_end.uoffset - last_file_offset.uoffset) as usize
                            } else {
                                read.file_end.uoffset as usize
                            };
                        } else {
                            read_bytes += read.file_end.uoffset as usize;
                        }

                        maybe_last_file_offset = Some(read.file_end);

                        if read_bytes >= self.block_size {
                            for (id, range) in &intervals {
                                let target_name = &target_names[*id as usize];
                                dbg!(target_name);
                                self.store.store(target_name.as_str(), range);
                            }

                            intervals.clear();

                            read_bytes -= self.block_size;
                        }
                    }
                },
                Ok(None) =>
                    break,
                Err(err) =>
                    return Err(err)
            }
        }

        Ok(())
    }
}