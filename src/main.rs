use cli_commands::CliCommand;
use constants::{GIT_BASE_DIR, GIT_HEAD_FILE, GIT_OBJECTS_DIR, GIT_REFS_DIR};
use git_object::tree_line::TreeLines;
use git_object::{GitObject, GIT_OBJECT_TYPE_BLOB};
use std::env;
use std::fs;

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
    let GitObject::Blob(content_string) = GitObject::from_hash(blob_hash)? else {
        return Err(anyhow::anyhow!("Invalid blob hash"));
    };
    print!("{}", content_string);
    Ok(())
}

fn execute_hash_object_command(file_path: &str) -> anyhow::Result<()> {
    // Read file content
    let file_bytes = fs::read(file_path)?;
    // Write as object
    let object = GitObject::new(GIT_OBJECT_TYPE_BLOB, file_bytes.as_slice())?;
    let hash = object.write_to_fs()?;
    print!("{hash}");
    Ok(())
}

fn execute_list_tree_command(tree_hash: &str) -> anyhow::Result<()> {
    // Checks
    if tree_hash.len() < 6 {
        return Err(anyhow::anyhow!("Invalid tree hash"));
    }
    // Create object
    let GitObject::Tree(lines) = GitObject::from_hash(tree_hash)? else {
        return Err(anyhow::anyhow!("Invalid tree hash"));
    };
    let TreeLines(lines) = lines;
    // Print
    lines.iter().for_each(|line| println!("{}", line.path));
    Ok(())
}
