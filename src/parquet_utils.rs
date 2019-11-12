/// Helper functions to deal with the (rather lowlevel) Parquet API
use parquet::{
	column::{
		reader::{ColumnReader, ColumnReaderImpl},
		writer::{ColumnWriter, ColumnWriterImpl},
	},
	data_type::DataType,
	file::writer::RowGroupWriter,
	record::{Row, RowAccessor},
};

/// Copy all values for a given column from the reader to the writer
pub fn copy_column(reader: &mut ColumnReader, writer: &mut ColumnWriter) {
	match (reader, writer) {
		(ColumnReader::BoolColumnReader(reader), ColumnWriter::BoolColumnWriter(writer)) => {
			copy_column_typed(reader, writer)
		}
		(ColumnReader::Int32ColumnReader(reader), ColumnWriter::Int32ColumnWriter(writer)) => {
			copy_column_typed(reader, writer)
		}
		(ColumnReader::Int64ColumnReader(reader), ColumnWriter::Int64ColumnWriter(writer)) => {
			copy_column_typed(reader, writer)
		}
		(ColumnReader::Int96ColumnReader(reader), ColumnWriter::Int96ColumnWriter(writer)) => {
			copy_column_typed(reader, writer)
		}
		(ColumnReader::FloatColumnReader(reader), ColumnWriter::FloatColumnWriter(writer)) => {
			copy_column_typed(reader, writer)
		}
		(ColumnReader::DoubleColumnReader(reader), ColumnWriter::DoubleColumnWriter(writer)) => {
			copy_column_typed(reader, writer)
		}
		(
			ColumnReader::ByteArrayColumnReader(reader),
			ColumnWriter::ByteArrayColumnWriter(writer),
		) => copy_column_typed(reader, writer),
		(
			ColumnReader::FixedLenByteArrayColumnReader(reader),
			ColumnWriter::FixedLenByteArrayColumnWriter(writer),
		) => copy_column_typed(reader, writer),
		_ => unimplemented!(),
	}
}

/// Read a String from the row
pub fn get_string<'a>(row: &'a Row, i: usize) -> Option<&'a str> {
	row.get_string(i).map(|s| s.as_str()).ok()
}

/// Read a long from the row
pub fn get_long(row: &Row, i: usize) -> Option<i64> {
	row.get_long(i).ok()
}

/// Write an OPTIONAL INT64 column to the parquet
pub fn write_i64_column(
	group_writer: &mut Box<dyn RowGroupWriter>,
	values: &Vec<i64>,
	definitions: &Vec<i16>,
	repetitions: &Vec<i16>,
) {
	dbg!("write column", values.len(), definitions.len());
	let mut column_writer = group_writer.next_column().unwrap().unwrap();
	match &mut column_writer {
		ColumnWriter::Int64ColumnWriter(column_writer) => {
			let num_written = column_writer
				.write_batch(&values, Some(&definitions), Some(&repetitions))
				.unwrap();
			assert_eq!(num_written, values.len());
		}
		_ => unimplemented!(),
	};
	group_writer.close_column(column_writer).unwrap();
}

/// Internal (typed) implementation for copy_column
fn copy_column_typed<T: DataType>(
	reader: &mut ColumnReaderImpl<T>,
	writer: &mut ColumnWriterImpl<T>,
) {
	let batch_size = 10_000;
	let mut def_levels = vec![0; batch_size];
	let mut rep_levels = vec![0; batch_size];
	let mut values: Vec<T::T> = vec![Default::default(); batch_size];

	loop {
		let (read_values, read_levels) = reader
			.read_batch(
				batch_size,
				Some(&mut def_levels),
				Some(&mut rep_levels),
				&mut values,
			)
			.unwrap();

		if read_values == 0 && read_levels == 0 {
			break;
		}

		let num_written = writer
			.write_batch(
				&values[..read_values],
				Some(&def_levels[..read_levels]),
				Some(&rep_levels[..read_levels]),
			)
			.unwrap();
		assert_eq!(read_values, num_written);
	}
}
