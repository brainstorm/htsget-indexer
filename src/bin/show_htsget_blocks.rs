use diesel;

use crate::db::models::*;
use crate::db::schema;

use self::diesel::prelude::*;

fn main() {
    use self::schema::htsget_blocks::dsl::*;

    let connection = establish_connection();
    let results = htsget_blocks
        .limit(5)
        .load::<Htsget_block>(&connection)
        .expect("Error loading htsget blocks");

    println!("Displaying {} htsget_blocks", results.len());
    for block in htsget_blocks {
        println!("{}", block.bam_id);
        println!("----------\n");
        println!("{}", block.byte_start);
    }
}