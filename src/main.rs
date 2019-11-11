use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

use parquet::file::reader::{FileReader, SerializedFileReader, ParquetReader};
use parquet::record::{Row, ListAccessor, RowAccessor};

use parquet_derive::ParquetRecordWriter;
use parquet::file::writer::RowGroupWriter;

trait RecordWriter<T> {
    fn write_to_row_group(&self, row_group_writer: &mut Box<dyn RowGroupWriter>);
}

#[derive(ParquetRecordWriter)]
struct MyStruct {
    start: u64,
    end: u64,
    byte_start: u64,
    byte_end: u64
}

fn write_parquet(fname: &str) {
// Initialize your parquet file
    let mut writer = SerializedFileWriter::new(file, schema, props).unwrap();
    let mut row_group = writer.next_row_group().unwrap();

// Build up your records
    let chunks = vec![MyStruct{...}];

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

//fn create_fields_index<T: ParquetReader>(reader: &SerializedFileReader<T>) -> HashMap<String, usize> {
//    let metadata = reader.metadata();
//    let mut map = HashMap::new();
//    for i in 0..metadata.num_row_groups() {
//        //let group = metadata.row_group(i);
//        //println!("{}", group);
//        //map.insert(col, i);
//    }
//    return map;
//}

fn main() {
    read_parquet("tests/data/htsnexus_test_NA12878.parquet")
}
