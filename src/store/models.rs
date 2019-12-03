use crate::store::schema::{ htsget_blocks };
use diesel::sql_types::{Numeric, BigInt};

#[derive(Queryable, Insertable)]
#[table_name="htsget_blocks"]
pub struct HtsgetBlock {
    pub bam_id: String,
    pub target_name: String,
    pub bytes_start: BigInt,
    pub bytes_end: BigInt,
    pub seq_start: BigInt,
    pub seq_end: BigInt,
}