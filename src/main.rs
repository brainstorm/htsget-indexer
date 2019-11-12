mod schemas;

use std::rc::Rc;
use std::fs::File;
use std::path::Path;

use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{RowAccessor};

use parquet::{
    file::{
        properties::WriterProperties,
        writer::{FileWriter, SerializedFileWriter},
    },
    schema::parser::parse_message_type,
};

use schemas::RecordWriter;

fn write_parquet(fname: &str) {
    let schema_str = schemas::ADAM_ALIGNMENT;
    read_parquet("tests/data/htsnexus_test_NA12878.parquet");
    //seek_voffset("tests/data/htsnexus_test_NA12878.bam");

    // Build up your records
    let chunks = vec![schemas::AdamBAM{
        start: 0,
        end: 0,
        byte_start: 0,
        byte_end: 0
    }];

    let schema = Rc::new(parse_message_type(schema_str).unwrap());
    let props = Rc::new(WriterProperties::builder().build());

    // Initialize your parquet file
    let mut writer = SerializedFileWriter::new(File::create(&Path::new(fname)).unwrap(), schema, props).unwrap();
    let mut row_group = writer.next_row_group().unwrap();

    // The derived `RecordWriter` takes over here
    (&chunks[..]).write_to_row_group(&mut row_group);

    writer.close_row_group(row_group).unwrap();
    writer.close().unwrap();
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
    write_parquet("tests/data/htsnexus_test_NA12878_with_voffsets.parquet");
}
