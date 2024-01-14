use crate::{
    compressor::Compressor,
    git_object::{
        tree_line::{Mode, TreeLine},
        GitObject, GIT_OBJECT_TYPE_BLOB, GIT_OBJECT_TYPE_TREE,
    },
    hasher::create_hash,
};
use std::{fs, vec};

const GIT_BASE_DIR: &str = ".git";
const GIT_OBJECTS_DIR: &str = ".git/objects";
const GIT_REFS_DIR: &str = ".git/refs";
const GIT_HEAD_FILE: &str = ".git/HEAD";

pub struct FsUtils {}

impl FsUtils {
    pub fn init_dir() -> anyhow::Result<()> {
        fs::create_dir(GIT_BASE_DIR)?;
        fs::create_dir(GIT_OBJECTS_DIR)?;
        fs::create_dir(GIT_REFS_DIR)?;
        fs::write(GIT_HEAD_FILE, "ref: refs/heads/master\n")?;
        Ok(())
    }
}

// Read
impl FsUtils {
    fn read_hash_bytes(hash: &str) -> anyhow::Result<Vec<u8>> {
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
        // Read bytes
        let file_bytes = fs::read(file_path)?;
        Ok(file_bytes)
    }

    pub fn read_object_with_hash(hash: &str) -> anyhow::Result<GitObject> {
        // Read file
        let bytes = FsUtils::read_hash_bytes(hash)?;
        // Decompress
        let decompressed_bytes_vec = Compressor::decompress(&bytes)?;
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
}

// Write
impl FsUtils {
    pub fn write_tree(path: &str) -> anyhow::Result<Vec<u8>> {
        let mut tree_bytes: Vec<u8> = vec![];

        let entries = fs::read_dir(path)?;
        let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.path());

        for entry in entries {
            // Get path
            let entry_path = entry.path();

            // If it's the git directory, skip it
            if entry_path.ends_with(GIT_BASE_DIR) {
                continue;
            }
            let file_name = entry.file_name();
            let Ok(file_name_string) = file_name.into_string() else {
                continue;
            };

            // Directory
            if entry_path.is_dir() {
                let Ok(entry_path_string) = entry_path.clone().into_os_string().into_string()
                else {
                    continue;
                };
                let hash = Self::write_tree(entry_path_string.as_str())?;
                let line = TreeLine::new(Mode::Folder, file_name_string, hash);
                tree_bytes.extend(line.to_bytes());
                continue;
            }

            // File
            let file_bytes = fs::read(entry_path)?;
            let hash = Self::write_raw_object(GIT_OBJECT_TYPE_BLOB, &file_bytes)?;
            let line = TreeLine::new(Mode::File, file_name_string, hash);
            tree_bytes.extend(line.to_bytes());
        }

        // Write tree
        let hash = Self::write_raw_object(GIT_OBJECT_TYPE_TREE, &tree_bytes)?;

        Ok(hash)
    }

    fn get_header(object_type: &str, content_bytes: &[u8]) -> String {
        format!("{object_type} {}\0", content_bytes.len())
    }

    pub fn write_object(object: &GitObject) -> anyhow::Result<String> {
        // Create object content

        // let (hash, compressed_data) = object.to_raw()?;

        let object_type = object.object_type();
        let content_bytes = match object {
            GitObject::Blob(content_string) => content_string.as_bytes().to_vec(),
            GitObject::Tree(tree_lines) => tree_lines.to_bytes(),
        };
        let header = Self::get_header(object_type.as_str(), &content_bytes);
        let content = [header.as_bytes(), &content_bytes].concat();

        // Hash
        let hash = hex::encode(create_hash(&content));

        // Compress
        let compressed_data = Compressor::compress(&content)?;

        // Write
        let dir_path = format!("{GIT_OBJECTS_DIR}/{}", &hash[..2]);
        if !fs::metadata(dir_path.clone()).is_ok() {
            fs::create_dir(&dir_path)?;
        }
        let object_path = format!("{dir_path}/{}", &hash[2..]);
        fs::write(&object_path, compressed_data)?;
        // Return hash
        Ok(hash)
    }

    fn write_raw_object(object_type: &str, object_content: &[u8]) -> anyhow::Result<Vec<u8>> {
        let header = Self::get_header(object_type, object_content);
        let object_bytes = [header.as_bytes(), &object_content].concat();

        // Hash
        let hash = create_hash(&object_bytes);
        let hash_hex = hex::encode(&hash);

        // Compress
        let compressed_object_bytes = Compressor::compress(&object_bytes)?;

        // Write
        let dir_path = format!("{GIT_OBJECTS_DIR}/{}", &hash_hex[..2]);
        if !fs::metadata(&dir_path).is_ok() {
            fs::create_dir(&dir_path)?;
        }
        let object_path = format!("{dir_path}/{}", &hash_hex[2..]);
        fs::write(&object_path, compressed_object_bytes)?;

        Ok(hash)
    }
}
