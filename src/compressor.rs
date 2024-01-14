use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use std::io::{Read, Write};

pub struct Compressor {}

impl Compressor {
    pub fn compress(bytes: &[u8]) -> anyhow::Result<Vec<u8>> {
        let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&bytes)?;
        Ok(encoder.finish()?)
    }

    pub fn decompress(bytes: &[u8]) -> anyhow::Result<Vec<u8>> {
        let mut decompressed_bytes = vec![];
        let mut decoder = ZlibDecoder::new(bytes);
        decoder.read_to_end(&mut decompressed_bytes)?;
        Ok(decompressed_bytes)
    }
}
