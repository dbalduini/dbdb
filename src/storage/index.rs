use super::fs::get_or_create_file;
use std::path::PathBuf;

use std::io::prelude::*;

/// Append to an append-only-index the `hash -> filename` relationship.
/// 
/// The index is used to quickly search root hashes by filename.
pub fn append_only_index(root: &str, filename: &str) -> std::io::Result<()>  {
    let mut file = get_or_create_file(PathBuf::from("index"), true)?;
    writeln!(&mut file, "{} {}", root, filename)?;
    Ok(())
}
