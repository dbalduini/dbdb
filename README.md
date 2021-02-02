## dbdb

An object storage inspired on Git and IPFS.

# List of commands

- `add <FILE>`
- `cat <HASH>`

## plumbing

- `hash-object <FILE>`
- `cat-block <[HASH]>`


# Internals

## blocks

Split a file into chunks of 1024b

-b Block size in kilobytes. Must be power of two. Defaults to 4 KB

./dbdb chunk-file -b 256 

WiredTiger, MinIO, RocksDB
block_size=4 KB

IPFS
block_size=256 KB

HDFS
block_size=128 MB

## compression

Compression is Snappy to be compatible with Spark parquet files
