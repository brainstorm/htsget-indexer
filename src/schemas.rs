use parquet::file::writer::RowGroupWriter;
use parquet_derive::{ParquetRecordWriter};

pub trait RecordWriter<T> {
    fn write_to_row_group(&self, row_group_writer: &mut Box<dyn RowGroupWriter>);
}

#[derive(ParquetRecordWriter)]
pub struct AdamBAM {
    pub start: i64,
    pub end: i64,
    pub byte_start: i64,
    pub byte_end: i64
}

pub const ADAM_ALIGNMENT: &str = "message schema {
        REQUIRED INT64         start;
        REQUIRED INT64         end;
        REQUIRED INT64         byte_start;
        REQUIRED INT64         byte_end;
}";

//pub const ADAM_ALIGNMENT: &str = "message schema {
//        REQUIRED BINARY        referenceName (UTF8);
//        REQUIRED INT64         start;
//        REQUIRED INT64         originalstart;
//        REQUIRED INT64         end;
//        REQUIRED INT32         mappingQuality;
//        REQUIRED BINARY        readName (UTF8);
//        REQUIRED BINARY        sequence (UTF8);
//        REQUIRED BINARY        quality (UTF8);
//        REQUIRED BINARY        cigar (UTF8);
//        REQUIRED INT32         basesTrimmedFromStart;
//        REQUIRED INT32         basesTrimmedFromEnd;
//        REQUIRED BOOLEAN       readPaired;
//        REQUIRED BOOLEAN       properPair;
//        REQUIRED BOOLEAN       readMapped;
//        REQUIRED BOOLEAN       mateMapped;
//        REQUIRED BOOLEAN       failedVendorQualityChecks;
//        REQUIRED BOOLEAN       duplicateRead;
//        REQUIRED BOOLEAN       readNegativeStrand;
//        REQUIRED BOOLEAN       mateNegativeStrand;
//        REQUIRED BOOLEAN       primaryAlignment;
//        REQUIRED BOOLEAN       secondaryAlignment;
//        REQUIRED BOOLEAN       supplementaryAlignment;
//        REQUIRED BINARY        mismatchingPositions (UTF8);
//        REQUIRED BINARY        readGroupId (UTF8);
//        REQUIRED BINARY        readGroupSampleId (UTF8);
//        REQUIRED INT32         mateAlignmentStart;
//        REQUIRED BINARY        mateReferenceName (UTF8);
//        REQUIRED INT32         insertSize;
//        REQUIRED INT32         readInFragment;
//        REQUIRED BINARY        attributes (UTF8);
//        REQUIRED INT64         byte_start;
//        REQUIRED INT64         byte_end;
//}";
