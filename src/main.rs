use cli_commands::CliCommand;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

use crate::constants::GIT_BASE_DIR;
use crate::constants::GIT_HEAD_FILE;
use crate::constants::GIT_OBJECTS_DIR;
use crate::constants::GIT_REFS_DIR;
use crate::git_object::GitObject;
use crate::git_object::GIT_OBJECT_TYPE_BLOB;

mod cli_commands;
mod constants;
mod git_object;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = CliCommand::from_string(&args[1]);

    match command {
        CliCommand::Init => execute_init_command()?,
        CliCommand::CatFile => execute_cat_file_command(&args[3])?,
        CliCommand::HashObject => execute_hash_object_command(&args[3])?,
        CliCommand::ListTree => execute_list_tree_command(&args[3])?,
    }

    Ok(())
}

fn execute_init_command() -> anyhow::Result<()> {
    fs::create_dir(GIT_BASE_DIR)?;
    fs::create_dir(GIT_OBJECTS_DIR)?;
    fs::create_dir(GIT_REFS_DIR)?;
    fs::write(GIT_HEAD_FILE, "ref: refs/heads/master\n")?;
    println!("Initialized git directory");
    Ok(())
}

fn execute_cat_file_command(blob_hash: &str) -> anyhow::Result<()> {
    let GitObject::Blob(content_string) = GitObject::from_hash(blob_hash)?;
    print!("{}", content_string);
    Ok(())
}

fn execute_hash_object_command(file_path: &str) -> anyhow::Result<()> {
    // Read file content
    let file_bytes = fs::read_to_string(file_path)?;
    // Write as object
    let object = GitObject::new(GIT_OBJECT_TYPE_BLOB, file_bytes.as_str())?;
    let hash = object.write_to_fs()?;
    print!("{hash}");
    Ok(())
}

fn execute_list_tree_command(tree_hash: &str) -> anyhow::Result<()> {
    // Checks
    if tree_hash.len() < 6 {
        return Err(anyhow::anyhow!("Invalid tree hash"));
    }

    // // Read
    // let mut dir_iterator = fs::read_dir(format!("{GIT_OBJECTS_DIR}/{}/", &tree_hash[..2]))?;
    // let Some(Ok(file_fs_dir_entry)) = dir_iterator.find(|entry| {
    //     let Ok(entry) = entry.as_ref() else {
    //         return false;
    //     };
    //     let Ok(entry_name) = entry.file_name().into_string() else {
    //         return false;
    //     };
    //     entry_name.starts_with(&tree_hash[2..])
    // }) else {
    //     return Err(anyhow::anyhow!("Invalid tree hash"));
    // };
    // let file_path = file_fs_dir_entry.path();
    // let Some(file_path) = file_path.to_str() else {
    //     return Err(anyhow::anyhow!("Invalid tree hash"));
    // };
    // let file_data = fs::read(file_path)?;

    // // Decompress
    // let mut decoder = ZlibDecoder::new(file_data.as_slice());
    // let mut blob_string = String::new();
    // decoder.read_to_string(&mut blob_string)?;

    Ok(())
}
