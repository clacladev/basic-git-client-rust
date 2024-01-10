use crate::constants::GIT_OBJECTS_DIR;
use flate2::read::ZlibDecoder;
use std::{fs, io::Read};

pub enum GitObject {
    Blob(String),
    // Tree(Tree),
}

impl GitObject {
    fn from_raw(object_type: &str, content: &str) -> anyhow::Result<Self> {
        match object_type {
            "blob" => Ok(GitObject::Blob(content.to_string())),
            // "tree" => Ok(GitObject::Tree(Tree::from_string(content)?)),
            _ => Err(anyhow::anyhow!("Invalid object type")),
        }
    }

    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        // Decompress
        let mut decoder = ZlibDecoder::new(bytes);
        let mut blob_string = String::new();
        decoder.read_to_string(&mut blob_string)?;
        // Get type
        let Some((object_type, blob_string)) = blob_string.split_once(' ') else {
            return Err(anyhow::anyhow!("Failed to read object type"));
        };
        // Get content
        let Some((_, content_string)) = blob_string.split_once('\0') else {
            return Err(anyhow::anyhow!("Failed to read object length"));
        };
        // Create object
        GitObject::from_raw(object_type, content_string)
    }

    fn from_path(path: &str) -> anyhow::Result<Self> {
        // Read
        let file_bytes = fs::read(path)?;
        // Create object
        GitObject::from_bytes(file_bytes.as_slice())
    }

    pub fn from_hash(hash: &str) -> anyhow::Result<Self> {
        // Checks
        if hash.len() < 6 {
            return Err(anyhow::anyhow!("Invalid tree hash"));
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
            return Err(anyhow::anyhow!("Invalid tree hash"));
        };
        // Create path
        let file_path: std::path::PathBuf = file_fs_dir_entry.path();
        let Some(file_path) = file_path.to_str() else {
            return Err(anyhow::anyhow!("Invalid tree hash"));
        };
        // Create object
        GitObject::from_path(file_path)
    }
}
