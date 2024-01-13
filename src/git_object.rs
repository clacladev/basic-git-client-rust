use self::tree_lines::TreeLines;
use crate::hasher::create_hash;
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use std::io::{Read, Write};

pub mod tree_line;
pub mod tree_lines;

pub const GIT_OBJECT_TYPE_BLOB: &str = "blob";
pub const GIT_OBJECT_TYPE_TREE: &str = "tree";

pub enum GitObject {
    Blob(String),
    Tree(TreeLines),
}

impl GitObject {
    pub fn object_type(&self) -> String {
        match self {
            GitObject::Blob(_) => GIT_OBJECT_TYPE_BLOB.to_string(),
            GitObject::Tree(_) => GIT_OBJECT_TYPE_TREE.to_string(),
        }
    }
}

impl GitObject {
    pub fn new(object_type: &str, content_bytes: &[u8]) -> anyhow::Result<Self> {
        match object_type {
            GIT_OBJECT_TYPE_BLOB => {
                let content_string = String::from_utf8_lossy(content_bytes).to_string();
                Ok(GitObject::Blob(content_string))
            }
            GIT_OBJECT_TYPE_TREE => {
                let lines = TreeLines::from_bytes(content_bytes)?;
                Ok(GitObject::Tree(lines))
            }
            _ => Err(anyhow::anyhow!(format!(
                "Invalid object type {}",
                object_type
            ))),
        }
    }
}

impl GitObject {
    pub fn from_object_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        // Decompress
        let mut decoder = ZlibDecoder::new(bytes);
        let mut decompressed_bytes_vec = vec![];
        decoder.read_to_end(&mut decompressed_bytes_vec)?;
        let mut decompressed_bytes = decompressed_bytes_vec.as_slice();

        // Parse
        let Some(space_index) = decompressed_bytes.iter().position(|&b| b == b' ') else {
            return Err(anyhow::anyhow!("Failed to read object type"));
        };
        let object_type = String::from_utf8_lossy(&decompressed_bytes[..space_index]).to_string();
        decompressed_bytes = &decompressed_bytes[(space_index + 1)..];

        let Some(null_index) = decompressed_bytes.iter().position(|&b| b == b'\0') else {
            return Err(anyhow::anyhow!("Failed to read object length"));
        };
        decompressed_bytes = &decompressed_bytes[(null_index + 1)..];

        // Create object
        Ok(GitObject::new(object_type.as_str(), decompressed_bytes)?)
    }

    pub fn to_raw(&self) -> anyhow::Result<(String, Vec<u8>)> {
        let object_type = self.object_type();
        let content_bytes = match self {
            GitObject::Blob(content_string) => content_string.as_bytes().to_vec(),
            GitObject::Tree(tree_lines) => tree_lines.to_bytes(),
        };
        let header = format!("{object_type} {}\0", content_bytes.len());
        let content = [header.as_bytes(), &content_bytes].concat();

        // Hash
        let hash = hex::encode(create_hash(&content));

        // Compress
        let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&content)?;
        let compressed_data = encoder.finish()?;

        Ok((hash, compressed_data))
    }
}
