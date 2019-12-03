use crate::store::schema::{ htsget_blocks };

#[derive(Queryable, Insertable)]
#[table_name="htsget_blocks"]
pub struct HtsgetBlock {
    pub bam_id: String,
    pub target_name: String,
    pub bytes_start: i64,
    pub bytes_end: i64,
    pub seq_start: i64,
    pub seq_end: i64,
}