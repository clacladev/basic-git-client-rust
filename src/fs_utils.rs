use crate::{
    constants::{GIT_BASE_DIR, GIT_OBJECTS_DIR},
    git_object::{
        tree_line::{TreeLine, TreeLines, TREE_LINE_MODE_FILE, TREE_LINE_MODE_FOLDER},
        GitObject,
    },
    hasher::create_hex_hash,
};
use std::{fs, vec};

pub struct FsUtils {}

impl FsUtils {
    pub fn read_bytes_for_hash(hash: &str) -> anyhow::Result<Vec<u8>> {
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

    pub fn write_to_fs(git_object: GitObject) -> anyhow::Result<String> {
        // Create object content
        let (hash, compressed_data) = git_object.to_raw()?;
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

    pub fn make_tree_lines(path: String) -> anyhow::Result<TreeLines> {
        let mut lines: Vec<TreeLine> = vec![];
        let dir_iterator = fs::read_dir(path)?;

        for entry in dir_iterator {
            let entry = entry?;

            // Get path
            let entry_path = entry.path();
            let Ok(entry_path_string) = entry_path.clone().into_os_string().into_string() else {
                continue;
            };

            // If it's the git directory, skip it
            if entry_path_string.ends_with(GIT_BASE_DIR) {
                continue;
            }

            let file_name = entry.file_name();
            let Ok(file_name_string) = file_name.into_string() else {
                continue;
            };

            // If it's a directory
            if entry_path.is_dir() {
                // Make a tree lines object for the directory
                let sub_dir_lines = Self::make_tree_lines(entry_path_string)?;
                let sub_dir_bytes = sub_dir_lines.to_bytes();
                let hash = create_hex_hash(&sub_dir_bytes);
                lines.push(TreeLine::new(
                    TREE_LINE_MODE_FOLDER,
                    file_name_string.as_str(),
                    hash.as_str(),
                ));
                continue;
            }

            // Make a line
            let file_bytes = fs::read(entry_path)?;
            let hash = create_hex_hash(&file_bytes);

            lines.push(TreeLine::new(
                TREE_LINE_MODE_FILE,
                file_name_string.as_str(),
                hash.as_str(),
            ));
        }

        Ok(TreeLines::new(&lines))
    }
}
