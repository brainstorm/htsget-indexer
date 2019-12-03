CREATE TABLE htsget_blocks (
  bam_id VARCHAR PRIMARY KEY,
  target_name VARCHAR NOT NULL,
  bytes_start BIGINT NOT NULL,
  bytes_end BIGINT NOT NULL,
  seq_start BIGINT NOT NULL,
  seq_end BIGINT NOT NULL
)
