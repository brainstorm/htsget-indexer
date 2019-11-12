## htsget Rust indexer

At the moment we get the ADAM parquet file, as converted via:

```shell
$ adam-submit transformAlignments input.bam output_dir
$ parquet-tools cat -j part-r-00000.gz.parquet | jq
```

And then add [virtual offsets](https://github.com/rust-bio/rust-htslib/pull/40/files) column/field for each of those reads, alongside the chromosomic positions, namely, for each record/read:

```
referenceName
start
end
(... all ADAM/BAM fields ...)
start_bytes
end_bytes
```

Ideally ADAM tools could populate those extra columns (optionally), but this is early days/work, stay tuned ;)

This code will change/simplify once `parquet-rs` implements parquet row write support, [currently in the works](https://github.com/apache/arrow/pull/4140)... for now using [`parquet_derive`](https://github.com/ccakes/parquet_derive) (unpublished) crate to patch that write support.