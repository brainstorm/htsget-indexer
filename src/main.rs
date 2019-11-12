mod parquet_utils;
mod schemas;

use std::fs::File;
use std::path::Path;
use std::rc::Rc;

use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;

use parquet::{
    basic::{Repetition, Type as BasicType},
    file::{
        properties::WriterProperties,
        writer::{FileWriter, SerializedFileWriter},
    },
    schema::{
        parser::parse_message_type,
        types::{GroupTypeBuilder, PrimitiveTypeBuilder},
    },
};

fn handle_parquet(in_fname: &str, out_fname: &str) {
    // Open input parquet file
    let in_file = File::open(&Path::new(in_fname)).unwrap();
    let reader = SerializedFileReader::new(in_file).unwrap();

    // Read input type
    let file_metadata = reader.metadata().file_metadata();
    let input_schema = file_metadata.schema();
    let input_fields = input_schema.get_fields();

    // Build output type
    let mut output_fields = input_fields.clone().to_vec();
    output_fields.push(Rc::new(
        PrimitiveTypeBuilder::new("byte_start", BasicType::INT64)
            .with_repetition(Repetition::REQUIRED)
            .build()
            .unwrap(),
    ));
    output_fields.push(Rc::new(
        PrimitiveTypeBuilder::new("byte_end", BasicType::INT64)
            .with_repetition(Repetition::REQUIRED)
            .build()
            .unwrap(),
    ));
    let output_schema = Rc::new(
        GroupTypeBuilder::new(input_schema.name())
            .with_fields(&mut output_fields)
            .build()
            .unwrap(),
    );

    // Prepare read projection
    let read_columns = parse_message_type(schemas::ADAM_READ_COLUMNS).unwrap();

    // Prepare output writer
    let out_file = File::create(&Path::new(out_fname)).unwrap();
    let output_props = Rc::new(WriterProperties::builder().build());
    let mut writer = SerializedFileWriter::new(out_file, output_schema, output_props).unwrap();

    // Enrich each row group
    for group_i in 0..reader.num_row_groups() {
        let group_reader = reader.get_row_group(group_i).unwrap();
        let mut group_writer = writer.next_row_group().unwrap();

        // Copy all columns from input to output
        for column_i in 0..group_reader.num_columns() {
            let mut column_reader = group_reader.get_column_reader(column_i).unwrap();
            let mut column_writer = group_writer.next_column().unwrap().unwrap();
            parquet_utils::copy_column(&mut column_reader, &mut column_writer);
            group_writer.close_column(column_writer).unwrap();
        }

        // Enrich each row
        for row in group_reader
            .get_row_iter(Some(read_columns.clone()))
            .unwrap()
        {
            let reference_name = row.get_string(0);
            let start = row.get_long(1);
            let end = row.get_long(2);
            dbg!(reference_name, start, end);
            break;
        }

        writer.close_row_group(group_writer).unwrap();
    }

    writer.close().unwrap();
}

fn main() {
    handle_parquet(
        "tests/data/htsnexus_test_NA12878.parquet",
        "tests/data/htsnexus_test_NA12878_with_voffsets.parquet",
    );
}
