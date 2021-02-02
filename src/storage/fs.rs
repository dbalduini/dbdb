use std::fs::create_dir_all;
use std::fs::File;
use std::path::PathBuf;

const WORK_DIR: &str = "data";

// partitions the data by the first two bytes (chars)
pub fn get_path_from_hash(hash: &str) -> PathBuf {
    PathBuf::from("objects").join(&hash[..2]).join(&hash[2..])
}

// Compute path under workdir
pub fn work_dir_path(path: PathBuf) -> PathBuf {
    PathBuf::from(WORK_DIR).join(path)
}

pub fn get_or_create_file(path: PathBuf, create: bool) -> std::io::Result<File> {
    if path.parent().is_some() {
        get_or_create_dir(path.parent().unwrap().to_path_buf(), true);
    }

    let path = work_dir_path(path);

    if create {
        File::create(path)
    } else {
        File::open(path)
    }
}

pub fn get_or_create_dir(p: PathBuf, mkdir: bool) -> PathBuf {
    let path = work_dir_path(p);

    if path.exists() {
        if path.is_dir() {
            return path;
        }
        panic!("path {} is not a directory", path.display());
    }

    if mkdir {
        create_dir_all(&path).unwrap();
    }
    path
}
