table! {
    htsget_blocks (id) {
        bam_id -> Varchar,
        target_name -> Varchar,
        bytes_start -> Int4,
        bytes_end -> Int4,
        seq_start -> Int4,
        seq_end -> Int4,
    }
}
