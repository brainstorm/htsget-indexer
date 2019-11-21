use std::rc::Rc;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{RowAccessor};

use parquet::{
    errors::ParquetError,
    file::{
        properties::WriterProperties,
        writer::{FileWriter, SerializedFileWriter},
    },
    schema::parser::parse_message_type,
};

use crate::data::schemas::{ RecordWriter, AdamBAM, ADAM_ALIGNMENT };
use crate::data::voffsets::Voffsets;
use parquet::record::reader::RowIter;

pub enum Error {
    SchemaParse(ParquetError),
    CreateFile(std::io::Error),
    CreateWriter(ParquetError),
    CreateWriterGroup(ParquetError),
    // WriteError(ParquetError),
    CloseWriterGroup(ParquetError),
    CloseWriter(ParquetError),
}

pub fn write_parquet(fname: &str, fh: Result<RowIter, ParquetError>, voffsets: HashMap<i32, Voffsets>) -> Result<(), Error> {

    let mut chunks: Vec<AdamBAM>;

    // Reconstruct our ADAM parquet with voffsets added to it
    while let record = fh.unwrap().next().unwrap() {
        chunks.push(
            AdamBAM {
                referenceName: record.get_string(0),
                start: record.get_long(1),
                originalstart: record.get_long(2),
                end: record.get_long(3),
                mappingQuality: record.get_int(4),
                readName: record.get_string(5),
                sequence: record.get_string(6),
                quality: record.get_string(7),
                cigar: record.get_string(8),
                basesTrimmedFromStart: record.get_string(9),
                basesTrimmedFromEnd: record.get_string(10),
                readPaired: record.get_bool(11),
                properPair: record.get_bool(12),
                readMapped: record.get_bool(13),
                mateMapped: record.get_bool(14),
                failedVendorQualityChecks: record.get_bool(15),
                duplicateRead: record.get_bool(16),
                readNegativeStrand: record.get_bool(17),
                mateNegativeStrand: record.get_bool(18),
                primaryAlignment: record.get_bool(19),
                secondaryAlignment: record.get_bool(20),
                supplementaryAlignment: record.get_string(21),
                mismatchingPositions: record.get_string(22),
                readGroupId: record.get_string(23),
                readGroupSampleId: record.get_string(24),
                mateAlignmentStart: record.get_int(25),
                mateReferenceName: record.get_string(26),
                insertSize: record.get_int(27),
                readInFragment: record.get_int(28),
                attributes: record.get_string(29),
                byte_start: voffsets[record.get_long(1)],
                byte_end: voffsets[record.get_long(3)],
            },
        );
    }

    let schema = Rc::new(
        parse_message_type(ADAM_ALIGNMENT)
            .map_err(|err| Error::SchemaParse(err))?
    );

    let props = Rc::new(WriterProperties::builder().build());

    let output_file = File::create(&Path::new(fname))
        .map_err(|err| Error::CreateFile(err))?;

    let mut writer = SerializedFileWriter::new(output_file, schema, props)
        .map_err(|err| Error::CreateWriter(err))?;

    let mut row_group = writer.next_row_group()
        .map_err(|err| Error::CreateWriterGroup(err))?;

    // The derived `RecordWriter` takes over here
    (&chunks[..]).write_to_row_group(&mut row_group);

    writer.close_row_group(row_group)
        .map_err(|err| Error::CloseWriterGroup(err))?;

    writer.close()
        .map_err(|err| Error::CloseWriter(err))
}

pub fn read_parquet(fname: &str) {
    let file = File::open(&Path::new(fname)).unwrap();
    let reader = SerializedFileReader::new(file).unwrap();
    let mut iter = reader.get_row_iter(None).unwrap();
    while let Some(record) = iter.next() {
        let genomic_start = record.get_long(1);
        let genomic_end = record.get_long(3);
        println!("{:#?} {:#?}", genomic_start, genomic_end);
        break;
    }
}

pub fn get_parquet_fh(fname: &str) -> Result<RowIter, ParquetError> {
    let file = File::open(&Path::new(fname)).unwrap();
    let reader = SerializedFileReader::new(file).unwrap();
    return reader.get_row_iter(None)
}