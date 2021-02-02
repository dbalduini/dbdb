use clap::{App, Arg, SubCommand};

use std::io::prelude::*;
use storage::block::{Block, BlockManager};
use storage::index::append_only_index;
use storage::tree::{read_tree, MerkleTree};

mod storage;

fn main() {
    let matches = App::new("dbdb")
        .version("0.1")
        .about("Object storage inspired on git and ipfs")
        .subcommand(
            SubCommand::with_name("hash-object")
                .arg(Arg::with_name("w").short("w"))
                .arg(Arg::with_name("FILE").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("cat-block").arg(Arg::with_name("HASH").required(true).multiple(true)),
        )
        .subcommand(
            SubCommand::with_name("add").arg(Arg::with_name("FILE").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("cat").arg(Arg::with_name("HASH").required(true).index(1)),
        )
        .get_matches();

    match matches.subcommand() {
        ("add", Some(args)) => cmd_add(args.value_of("FILE").unwrap()),
        ("cat", Some(args)) => cmd_cat(args.value_of("HASH").unwrap()),
        ("hash-object", Some(args)) => {
            cmd_hash_object(args.value_of("FILE").unwrap(), args.is_present("w"))
        }
        ("cat-block", Some(args)) => {
            if let Some(hash) = args.values_of("HASH") {
                let blocks = hash.collect::<Vec<_>>();
                cmd_cat_block(blocks);
            }
        }
        _ => println!("{}", matches.usage()),
    }
}

fn cmd_hash_object(path: &str, w: bool) {
    let bm = BlockManager::new();
    let blocks = bm.split(path, w);
    for hash in blocks {
        println!("{}", hash);
    }
}

fn cmd_cat_block(blocks: Vec<&str>) {
    let bm = BlockManager::new();
    for hash in blocks {
        let (n, chunk) = bm.read_chunk(hash);
        let block = Block::from(&chunk[..n]);
        let data = block.data();
        std::io::stdout().write_all(data).unwrap();
    }
}

fn cmd_add(path: &str) {
    let bm = BlockManager::new();

    let chunks = bm.split(path, true);
    let chunks = chunks.iter().map(|x| x.to_string()).collect();

    let mut tree = MerkleTree::new(chunks);
    let root_hash = tree.write().unwrap();

    append_only_index(&root_hash, "lorem.txt").unwrap();

    println!("{}", root_hash);
}

fn cmd_cat(root: &str) {
    // read the blocks from the tree
    let mut blocks = Vec::new();
    read_tree(root, &mut blocks);
    // join the blocks into the stdout
    let bm = BlockManager::new();
    let mut writer = std::io::stdout();
    bm.join(blocks, &mut writer);
}
