use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use super::compression::*;
use super::fs::*;
use super::hash::hash_block;

pub struct Block<'buf> {
    data: &'buf [u8],
}

const KB: usize = 1024;
const BLOCK_SIZE: usize = 8 * KB;

impl<'buf> Block<'buf> {
    pub fn from(data: &'buf [u8]) -> Self {
        Self { data }
    }

    pub fn hash(&self) -> String {
        hash_block(self.data)
    }

    pub fn write(&self, w: impl Write) {
        //let mut writer = w;
        compress_block(w, self.data);
        //writer.write_all(&self.data[..]).unwrap();
    }

    pub fn data(&self) -> &'buf [u8] {
        self.data
    }
}

pub struct BlockManager {}

impl BlockManager {
    pub fn new() -> Self {
        Self {}
    }

    /// Split a file into blocks of fixed size.
    /// If create is set to `true`, the blocks will be written to disk.
    /// 
    /// An ordered vector is returned with the hash of the blocks.
    pub fn split(&self, file: &str, create: bool) -> Vec<String> {
        let input_file = PathBuf::from(file);
        let mut file = File::open(input_file).unwrap();
        let mut buff = [0; BLOCK_SIZE];
        let mut chunks: Vec<String> = Vec::new();

        loop {
            let n = file.read(&mut buff[..]).unwrap();
            if n == 0 {
                break;
            }

            let block = Block::from(&buff[..n]);
            let hash = block.hash();
            if create {
                //println!("writing block: hash {} with {} bytes", hash, n);
                let p = get_path_from_hash(&hash);
                let file = get_or_create_file(p, true).unwrap();
                block.write(file);
            }

            chunks.push(hash);
        }

        chunks
    }

    pub fn read_chunk(&self, hash: &str) -> (usize, [u8; BLOCK_SIZE]) {
        let path = get_path_from_hash(hash);
        let file = get_or_create_file(path, false).unwrap();
        let mut buff = [0; BLOCK_SIZE];
        let n = uncompress_block(file, &mut buff).unwrap();
        (n, buff)
    }

    /// Read all blocks chunks given the vector of hash
    /// and join the contents on the writer.
    pub fn join(&self, hashes: Vec<String>, w: impl Write) {
        let mut writer = w;
        for hash in hashes {
            let (n, chunk) = self.read_chunk(&hash);
            let block = Block::from(&chunk[..n]);
            let data = block.data();            
            writer.write(data).unwrap();
        }
        writer.flush().unwrap();
    }
}
