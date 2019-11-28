use diesel;

fn main() {
    let connection = establish_connection();
    let results = htsget_blocks
        .limit(5)
        .load::<Htsget_block>(&connection)
        .expect("Error loading htsget blocks");

    println!("Displaying {} htsget_blocks", results.len());
    for block in results {
        println!("{}", block.bam_id);
        println!("----------\n");
        println!("{}", block.byte_start);
    }
}