use std::io::Write;

use flate2::write::ZlibEncoder;

pub struct Compressor {}

impl Compressor {
    pub fn compress(bytes: &[u8]) -> anyhow::Result<Vec<u8>> {
        let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&bytes)?;
        Ok(encoder.finish()?)
    }
}
