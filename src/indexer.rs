use std::path::Path;
use std::collections::HashMap;
use theban_interval_tree::{insert};

use rust_htslib::bam::{Reader, Record, Read};

struct  Voffsets {
    coffset: u32,
    uoffset: u32,
}

pub fn seek_voffset(fname: &str) {
    let mut bam = Reader::from_path(&Path::new(fname)).ok().expect("Error opening file.");

    let mut voffset = Voffsets{ coffset: 0, uoffset: 0 };
    let mut pos_voffset = HashMap::new();
    let mut index = theban_interval_tree::IntervalTree::<Voffsets>::new();

    let mut offset = bam.tell();

    let mut rec = Record::new();
    loop {
        if !bam.read(&mut rec).expect("error reading bam") { break; }

        // Retrieve virtual offset
        offset = bam.tell();
        // Get compressed and uncompressed indexes from virtual offset
        let mut coffset = u32::from(u64::from(offset) >> 16);
        let mut uoffset = (u32::from(u64::from(offset) & 0xffff );
        voffset = Voffsets { coffset, uoffset };

        pos_voffset.insert(rec.pos(), voffset);
    }

    for (pos, voffsets) in pos_voffset {
        bam.seek(offset).unwrap();
        //bam.read(&mut rec).unwrap();
        //println!("{:?} {1: <10} {2: <10} {3: <10}", rec.tid(), pos, voffsets.uoffset, voffsets.coffset)
    }
}