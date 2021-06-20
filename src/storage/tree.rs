use super::fs::*;
use super::hash::*;

use std::io::prelude::*;

#[derive(Debug)]
pub enum Leaf<'f> {
    Single(&'f str),
    Pair(&'f str, &'f str),
}

/// In a MerkleTree, the leaf nodes are hash of the Data Blocks (aka. chunks) `L`.
/// The parent node is also a MerkleTree, and has at most two children.
/// 
/// The hash for each leaf node `h(L)` is computed by its content (content-addressing).
/// Then, the hash for each pair of leaf node h(h(L1) + h(L2)) is computed, creating a parent node.
/// 
/// Data nodes are prefixed with **block**, while parent nodes are prefixed with **tree**.
pub struct MerkleTree {
    hash_list: Vec<String>,
}

impl MerkleTree {
    pub fn new(hash_list: Vec<String>) -> Self {
        Self { hash_list }
    }

    /// Write this Merkle Tree into disk, returning the root node (the top hash).
    pub fn write(&mut self) -> Result<String, std::io::Error> {
        self.write_to_end("block")
    }

    fn write_to_end(&mut self, prefix: &str) -> Result<String, std::io::Error> {
        let xs = Self::split_into_tuples(&self.hash_list);
        let mut parent: Vec<String> = Vec::new();

        for x in xs {
            let parent_hash = match x {
                Leaf::Single(b) => {
                    let h = hash_str(b);
                    let path = get_path_from_hash(&h);
                    let mut file = get_or_create_file(path, true)?;
                    writeln!(&mut file, "{} {}", prefix, b)?;
                    h
                }
                Leaf::Pair(b1, b2) => {
                    let h = hash_tuple_2((b1, b2));
                    let path = get_path_from_hash(&h);
                    let mut file = get_or_create_file(path, true)?;
                    writeln!(&mut file, "{} {}\n{} {}", prefix, b1, prefix, b2)?;
                    h
                }
            };

            parent.push(parent_hash);
        }

        if parent.len() > 1 {
            self.hash_list.truncate(0);
            self.hash_list.append(&mut parent);
            self.write_to_end("tree")
        } else {
            Ok(parent[0].clone())
        }
    }

    fn split_into_tuples<'f>(blocks: &'f Vec<String>) -> Vec<Leaf<'f>> {
        let mut tuples = Vec::new();
        for i in (0..blocks.len() - 1).step_by(2) {
            tuples.push(Leaf::Pair(&blocks[i], &blocks[i + 1]));
        }
        if blocks.len() % 2 == 1 {
            tuples.push(Leaf::Single(&blocks[blocks.len() - 1]));
        }
        tuples
    }
}

/// Read a tree from a root node, appending the blocks hash into a vector.
pub fn read_tree(root: &str, hashes: &mut Vec<String>) {
    let path = get_path_from_hash(root);

    if let Ok(mut file) = get_or_create_file(path, false) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).ok();

        let lines: Vec<&str> = contents.split('\n').collect();

        for line in lines {
            if line.starts_with("tree") {
                // "tree "
                let node = line.split_at(5).1;
                read_tree(node, hashes);
            } else if line.starts_with("block") {
                // "block "
                let hash = line.split_at(6).1.to_string();
                hashes.push(hash);
            }
        }
    }
}
