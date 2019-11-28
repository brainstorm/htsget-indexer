CREATE TABLE htsget_blocks (
  bam_id VARCHAR NOT NULL,
  target_name VARCHAR NOT NULL,
  bytes_start INTEGER NOT NULL,
  bytes_end INTEGER NOT NULL,
  seq_start INTEGER NOT NULL,
  seq_end INTEGER NOT NULL 
)
