use std::path::Path;
//use std::collections::HashMap;
//use theban_interval_tree::{ insert };

use snafu::{ OptionExt, ResultExt };
use rust_htslib::bam::{ Reader, Record, Read };
use rust_htslib::bam::ext::BamRecordExtensions;

use crate::indexer::BlockIndexer;
use crate::errors::{ Result, Error };

pub struct Voffsets {
    /// compressed offset
    pub coffset: u32,
    /// uncompressed offset
    pub uoffset: u32,
}

impl Voffsets {
    /// Get a virtual offset from a file offset
    pub fn new(offset: i64) -> Self {
        // XXX: Revisit this byte manipulation
        let coffset = (offset >> 16) as u32;
        let uoffset = (offset & 0xffff ) as u32;
        Voffsets { coffset, uoffset }
    }
}

pub struct BamRead {
    pub voffset: Voffsets,
    pub start: i32,
    pub end: i32,
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

    fn read(&mut self) -> Result<BamRead> {
        let mut record = Record::new();
        match self.reader.read(&mut record) {
            Ok(true) => {
                let offset = self.reader.tell();
                Ok(
                    BamRead {
                        voffset: Voffsets::new(offset),
                        start: record.reference_start(),
                        end: record.reference_end(),
                    }
                )
            }
            Ok(false) => Err(Error::BamReadingUnknown {}),
            Err(source) => Err(Error::BamReading { source }),
        }
    }
}
