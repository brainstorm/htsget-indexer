use crate::store::schema::{ htsget_blocks };

#[derive(Queryable, Insertable)]
#[table_name="htsget_blocks"]
pub struct HtsgetBlock {
    pub bam_id: String,
    pub target_name: String,
    pub bytes_start: u64,
    pub bytes_end: u64,
    pub seq_start: u64,
    pub seq_end: u64,
}