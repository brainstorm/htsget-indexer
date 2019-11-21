mod data;

use crate::data::adam::{ read_parquet, write_parquet };

fn main() {
    read_parquet("tests/data/htsnexus_test_NA12878.parquet");
    //seek_voffset("tests/data/htsnexus_test_NA12878.bam");
    write_parquet("tests/data/htsnexus_test_NA12878_with_voffsets.parquet");
}
