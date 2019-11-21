use std::path::Path;
use std::collections::HashMap;
//use theban_interval_tree::{ insert };

use rust_htslib::bam::{ Reader, Record, Read };

pub struct Voffsets {
    coffset: u32,
    uoffset: u32,
}

pub fn all_voffsets(fname: &str) -> HashMap<i32, Voffsets> {
    let mut bam = Reader::from_path(&Path::new(fname)).ok().expect("Error opening file.");

    let mut pos_voffset = HashMap::new();
    //let mut index = theban_interval_tree::IntervalTree::<Voffsets>::new();

    let mut offset;

    let mut rec = Record::new();
    loop {
        if !bam.read(&mut rec).expect("error reading bam") { break; }

        // Retrieve virtual offset
        offset = bam.tell();
        // Get compressed and uncompressed indexes from virtual offset
        // XXX: Revisit this byte manipulation
        let coffset = (offset >> 16) as u32;
        let uoffset = (offset & 0xffff ) as u32;
        let voffset = Voffsets { coffset, uoffset };

        pos_voffset.insert(rec.pos(), voffset);
    }

//    for (pos, voffsets) in pos_voffset {
//        bam.seek(offset).unwrap();
//        bam.read(&mut rec).unwrap();
//        println!("{:?} {1: <10} {2: <10} {3: <10}", rec.tid(), pos, voffsets.uoffset, voffsets.coffset)
//    }

    return pos_voffset;
}