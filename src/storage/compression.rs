#![allow(dead_code)]

use std::io::{Read, Write};

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

pub fn compress_block(w: impl Write, buff: &[u8]) {
    compress_snappy(w, buff)
}

pub fn uncompress_block(r: impl Read, buff: &mut [u8]) -> std::io::Result<usize> {
    uncompress_snappy(r, buff)
}

fn compress_snappy(w: impl Write, buff: &[u8]) {
    let mut writer = snap::write::FrameEncoder::new(w);
    let mut reader: &[u8] = buff;
    std::io::copy(&mut reader, &mut writer).unwrap();
}

fn uncompress_snappy(r: impl Read, buff: &mut [u8]) -> std::io::Result<usize> {
    let mut reader = snap::read::FrameDecoder::new(r);
    reader.read(&mut buff[..])
}

fn compress_gzip(w: impl Write, buff: &[u8]) {
    let mut writer = ZlibEncoder::new(w, Compression::best());
    let mut reader: &[u8] = buff;
    std::io::copy(&mut reader, &mut writer).unwrap();
}

fn uncompress_gzip(r: impl Read, buff: &mut [u8]) -> std::io::Result<usize> {
    let mut reader = ZlibDecoder::new(r);
    reader.read(&mut buff[..])
}
