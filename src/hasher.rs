use sha1::{Digest, Sha1};

pub fn create_hash(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = <Sha1 as Digest>::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}
