mod data;

use crate::data::adam::{ get_parquet_fh, write_parquet };
use crate::data::voffsets::all_voffsets;

fn main() {
    let voffsets = all_voffsets("tests/data/htsnexus_test_NA12878.bam");

    let fh = get_parquet_fh("tests/data/htsnexus_test_NA12878.parquet");
    write_parquet("tests/data/htsnexus_test_NA12878_with_voffsets.parquet", fh, voffsets);
}
