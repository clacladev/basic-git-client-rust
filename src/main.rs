use cli_commands::CliCommand;
use git_object::tree_lines::TreeLines;
use git_object::{GitObject, GIT_OBJECT_TYPE_BLOB};
use std::env;
use std::fs;

use crate::fs_utils::FsUtils;

mod cli_commands;
mod compressor;
mod fs_utils;
mod git_object;
mod hasher;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = CliCommand::from_string(&args[1]);

    match command {
        CliCommand::Init => execute_init_command()?,
        CliCommand::CatFile => execute_cat_file_command(&args[3])?,
        CliCommand::HashObject => execute_hash_object_command(&args[3])?,
        CliCommand::ListTree => execute_list_tree_command(&args[3])?,
        CliCommand::WriteTree => execute_write_tree_command()?,
        CliCommand::CommitTree => execute_commit_tree_command(&args[2], &args[4], &args[6])?,
    }

    Ok(())
}

fn execute_init_command() -> anyhow::Result<()> {
    FsUtils::init_git_dir()?;
    println!("Initialized git directory");
    Ok(())
}

fn execute_cat_file_command(blob_hash: &str) -> anyhow::Result<()> {
    let bytes = FsUtils::read_bytes_for_hash(blob_hash)?;
    let GitObject::Blob(content_string) = GitObject::object_from_bytes(&bytes)? else {
        return Err(anyhow::anyhow!("Invalid blob hash"));
    };
    print!("{}", content_string);
    Ok(())
}

fn execute_hash_object_command(file_path: &str) -> anyhow::Result<()> {
    // Read file content
    let file_bytes = fs::read(file_path)?;
    // Write as object
    let object = GitObject::new(GIT_OBJECT_TYPE_BLOB, &file_bytes)?;
    let hash = FsUtils::write_to_fs(&object)?;
    // Print
    print!("{hash}");
    Ok(())
}

fn execute_list_tree_command(tree_hash: &str) -> anyhow::Result<()> {
    // Checks
    if tree_hash.len() < 6 {
        return Err(anyhow::anyhow!("Invalid tree hash"));
    }
    // Create object
    let bytes = FsUtils::read_bytes_for_hash(tree_hash)?;
    let GitObject::Tree(lines) = GitObject::object_from_bytes(&bytes)? else {
        return Err(anyhow::anyhow!("Invalid tree hash"));
    };
    let TreeLines(lines) = lines;
    // Print
    lines.iter().for_each(|line| println!("{}", line.path));
    Ok(())
}

fn execute_write_tree_command() -> anyhow::Result<()> {
    let hash = FsUtils::write_tree(".")?;
    println!("{}", hex::encode(hash));
    Ok(())
}

fn execute_commit_tree_command(
    _tree_hash: &str,
    _parent_commit_hash: &str,
    _message: &str,
) -> anyhow::Result<()> {
    Ok(())
}
