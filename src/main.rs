mod schemas;

use std::rc::Rc;
use std::fs::File;
use std::path::Path;

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

use schemas::RecordWriter;

pub enum Error {
    SchemaParse(ParquetError),
    CreateFile(std::io::Error),
    CreateWriter(ParquetError),
    CreateWriterGroup(ParquetError),
//    WriteError(ParquetError),
    CloseWriterGroup(ParquetError),
    CloseWriter(ParquetError),
}

fn write_parquet(fname: &str) -> Result<(), Error> {
    // Build up your records
    let chunks = vec![
        schemas::AdamBAM {
            start: 0,
            end: 0,
            byte_start: 0,
            byte_end: 0
        },
        schemas::AdamBAM {
            start: 10,
            end: 20,
            byte_start: 100,
            byte_end: 200
        },
    ];

    let schema = Rc::new(
        parse_message_type(schemas::ADAM_ALIGNMENT)
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

fn read_parquet(fname: &str) {
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

fn main() {
    read_parquet("tests/data/htsnexus_test_NA12878.parquet");
    //seek_voffset("tests/data/htsnexus_test_NA12878.bam");
    write_parquet("tests/data/htsnexus_test_NA12878_with_voffsets.parquet");
}
