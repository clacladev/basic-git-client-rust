use crate::constants::GIT_OBJECTS_DIR;
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{
    fs,
    io::{Read, Write},
};

use self::tree_line::TreeLines;

pub mod tree_line;

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

    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
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

    fn from_path(path: &str) -> anyhow::Result<Self> {
        let file_bytes = fs::read(path)?;
        GitObject::from_bytes(file_bytes.as_slice())
    }

    pub fn from_hash(hash: &str) -> anyhow::Result<Self> {
        // Checks
        if hash.len() < 6 {
            return Err(anyhow::anyhow!("Invalid hash"));
        }
        // Find file starting with hash
        let mut dir_iterator = fs::read_dir(format!("{GIT_OBJECTS_DIR}/{}/", &hash[..2]))?;
        let Some(Ok(file_fs_dir_entry)) = dir_iterator.find(|entry| {
            let Ok(entry) = entry.as_ref() else {
                return false;
            };
            let Ok(entry_name) = entry.file_name().into_string() else {
                return false;
            };
            entry_name.starts_with(&hash[2..])
        }) else {
            return Err(anyhow::anyhow!("Invalid hash"));
        };
        // Create path
        let file_path: std::path::PathBuf = file_fs_dir_entry.path();
        let Some(file_path) = file_path.to_str() else {
            return Err(anyhow::anyhow!("Invalid hash"));
        };
        // Create object
        GitObject::from_path(file_path)
    }
}

impl GitObject {
    fn to_raw(&self) -> anyhow::Result<(String, Vec<u8>)> {
        let object_type = self.object_type();
        let content_bytes = match self {
            GitObject::Blob(content_string) => content_string.as_bytes().to_vec(),
            GitObject::Tree(lines) => lines.to_bytes(),
        };
        let header = format!("{object_type} {}\0", content_bytes.len());
        let content = [header.as_bytes(), content_bytes.as_slice()].concat();

        // Hash
        let mut hasher = <Sha1 as Digest>::new();
        hasher.update(content.as_slice());
        let hash = hex::encode(hasher.finalize());

        // Compress
        let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&content)?;
        let compressed_data = encoder.finish()?;
        Ok((hash, compressed_data))
    }

    pub fn write_to_fs(&self) -> anyhow::Result<String> {
        // Create object content
        let (hash, compressed_data) = self.to_raw()?;
        // Write
        let dir_path = format!("{GIT_OBJECTS_DIR}/{}", &hash[..2]);
        fs::create_dir(&dir_path)?;
        let object_path = format!("{dir_path}/{}", &hash[2..]);
        fs::write(&object_path, compressed_data)?;
        Ok(hash)
    }
}
