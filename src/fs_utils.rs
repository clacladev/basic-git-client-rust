use crate::{
    constants::{GIT_BASE_DIR, GIT_OBJECTS_DIR},
    git_object::GitObject,
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
        fs::create_dir(&dir_path)?;
        let object_path = format!("{dir_path}/{}", &hash[2..]);
        fs::write(&object_path, compressed_data)?;
        Ok(hash)
    }

    pub fn ls_files(path: String) -> anyhow::Result<Vec<String>> {
        let mut files_paths: Vec<String> = vec![];
        let mut dir_iterator = fs::read_dir(path)?;

        while let Some(Ok(file_fs_dir_entry)) = dir_iterator.next() {
            // Get path
            let file_path = file_fs_dir_entry.path();
            let Ok(file_path_string) = file_path.clone().into_os_string().into_string() else {
                continue;
            };

            // If it's the git directory
            if file_path_string.ends_with(GIT_BASE_DIR) {
                continue;
            }

            // If it's a directory
            if file_path.is_dir() {
                // Get the files inside the directory
                let mut sub_dir_files_paths = FsUtils::ls_files(file_path_string)?;
                files_paths.append(&mut sub_dir_files_paths);
                continue;
            }

            // Remove the leading "./"
            let file_path_string = file_path_string[2..].to_string();

            // Save the path
            files_paths.push(file_path_string);
        }

        // Sort the paths
        files_paths.sort();

        Ok(files_paths)
    }
}
