use sha1::Sha1;

pub fn hash_block(buff: &[u8]) -> String {
    Sha1::from(buff).digest().to_string()
}

pub fn hash_str(s: &str) -> String {
    Sha1::from(s).digest().to_string()
}

pub fn hash_tuple_2<'f>(t: (&'f str, &'f str)) -> String {
    let mut sha1 = Sha1::new();
    sha1.update(t.0.as_bytes());
    sha1.update(t.1.as_bytes());
    sha1.digest().to_string()
}
