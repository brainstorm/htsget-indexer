use std::path::Path;
//use std::collections::HashMap;
//use theban_interval_tree::{ insert };

use snafu::{ OptionExt, ResultExt };
use rust_htslib::bam::{ Reader, Record, Read };
use rust_htslib::bam::ext::BamRecordExtensions;

use crate::indexer::BlockIndexer;
use crate::errors::{ Result, Error };

pub type TargetId = i32;
pub type Offset = u32;
pub type VirtualFileOffsets = i64;
pub type SeqPosition = i32;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct FileOffsets {
    /// compressed offset
    pub coffset: Offset,
    /// uncompressed offset
    pub uoffset: Offset,
}

impl FileOffsets {
    pub fn new(coffset: Offset, uoffset: Offset) -> Self {
        FileOffsets { coffset, uoffset }
    }

    /// Build from a virtual file offsets
    pub fn from_offset(offsets: VirtualFileOffsets) -> Self {
        let coffset = ((offsets >> 16) & 0xffff) as Offset;
        let uoffset = (offsets & 0xffff ) as Offset;
        FileOffsets { coffset, uoffset }
    }

    pub fn min(vo1: FileOffsets, vo2: FileOffsets) -> FileOffsets {
        if vo1.coffset == vo2.coffset {
            if vo1.uoffset <= vo2.uoffset {
                vo1
            }
            else {
                vo2
            }
        }
        else {
            if vo1.coffset <= vo2.coffset {
                vo1
            }
            else {
                vo2
            }
        }
    }

    pub fn max(vo1: FileOffsets, vo2: FileOffsets) -> FileOffsets {
        if vo1.coffset == vo2.coffset {
            if vo1.uoffset >= vo2.uoffset {
                vo1
            }
            else {
                vo2
            }
        }
        else {
            if vo1.coffset >= vo2.coffset {
                vo1
            }
            else {
                vo2
            }
        }
    }
}

#[derive(Debug)]
pub struct BamRead {
    pub target_id: TargetId,
    pub file_start: FileOffsets,
    pub file_end: FileOffsets,
    pub seq_start: SeqPosition,
    pub seq_end: SeqPosition,
}

pub struct BamReader {
    reader: Reader,
}

impl BamReader {
    pub fn new(fname: String) -> Result<Self> {
        Reader::from_path(&Path::new(fname.as_str()))
            .map_err(|source| Error::BamOpen { source })
            .map(|reader| BamReader { reader })
    }

    pub fn target_names(&self) -> Vec<String> {
        self.reader.header().target_names().into_iter()
            .map(|raw_name| String::from_utf8_lossy(raw_name).to_string())
            .collect()
    }

    pub fn read(&mut self) -> Result<Option<BamRead>> {
        let mut record = Record::new();
        let mut file_start = FileOffsets::from_offset(self.reader.tell());
        match self.reader.read(&mut record) {
            Ok(true) => {
                let length = record.inner().l_data as Offset;
                let file_end = FileOffsets::new(file_start.coffset,
                                                           file_start.uoffset + length);

                let read = BamRead {
                    file_start,
                    file_end,
                    target_id: record.tid(),
                    seq_start: record.reference_start(),
                    seq_end: record.reference_end(),
                };
                file_start = file_end;
                Ok(Some(read))
            }
            Ok(false) => Ok(None),
            Err(source) => Err(Error::BamReading { source }),
        }
    }
}
