use sha1::{Digest, Sha1};

pub fn create_hex_hash(bytes: &[u8]) -> String {
    let mut hasher = <Sha1 as Digest>::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}
