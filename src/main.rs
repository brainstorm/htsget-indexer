
use std::rc::Rc;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use parquet::file::reader::{FileReader, SerializedFileReader, ParquetReader};
use parquet::record::{Row, ListAccessor, RowAccessor};

use parquet_derive::{ParquetRecordWriter};

use parquet::{
    file::{
        properties::WriterProperties,
        writer::{FileWriter, SerializedFileWriter, RowGroupWriter},
    },
    schema::parser::parse_message_type,
};


trait RecordWriter<T> {
    fn write_to_row_group(&self, row_group_writer: &mut Box<dyn RowGroupWriter>);
}

#[derive(ParquetRecordWriter)]
struct MyStruct {
    start: i64,
    end: i64,
    byte_start: i64,
    byte_end: i64
}



fn write_parquet(fname: &str) {

    let schema_str = "message schema {
            REQUIRED INT64         start;
            REQUIRED INT64         end;
            REQUIRED INT64         byte_start;
            REQUIRED INT64         byte_end;
        }";

    let schema = Rc::new(parse_message_type(schema_str).unwrap());
    let props = Rc::new(WriterProperties::builder().build());

// Initialize your parquet file
    let mut writer = SerializedFileWriter::new(File::create(&Path::new(fname)).unwrap(), schema, props).unwrap();
    let mut row_group = writer.next_row_group().unwrap();

// Build up your records
    let chunks = vec![MyStruct{
        start: 0,
        end: 0,
        byte_start: 0,
        byte_end: 0
    }];

// The derived `RecordWriter` takes over here
    (&chunks[..]).write_to_row_group(&mut row_group);

    writer.close_row_group(row_group).unwrap();
    writer.close().unwrap();
}

fn read_parquet(fname: &str) {
    let file = File::open(&Path::new(fname)).unwrap();
    let reader = SerializedFileReader::new(file).unwrap();
    let metadata = reader.metadata();
    let mut iter = reader.get_row_iter(None).unwrap();
    while let Some(record) = iter.next() {
        let genomic_start = record.get_long(1);
        let genomic_end = record.get_long(3);
        //seek_voffset("tests/data/htsnexus_test_NA12878.parquet");
        println!("{:#?}", record);
        break;
    }
}

fn main() {
    read_parquet("tests/data/htsnexus_test_NA12878.parquet");
    write_parquet("tests/data/htsnexus_test_NA12878_with_voffsets.parquet");
}
