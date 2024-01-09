#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Read;
use std::io::Write;

use cli_commands::CliCommand;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};

use crate::constants::GIT_BASE_DIR;
use crate::constants::GIT_HEAD_FILE;
use crate::constants::GIT_OBJECTS_DIR;
use crate::constants::GIT_REFS_DIR;

mod cli_commands;
mod constants;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = CliCommand::from_string(&args[1]);

    match command {
        CliCommand::Init => execute_init_command()?,
        CliCommand::CatFile => execute_cat_file_command(&args[3])?,
        CliCommand::HashObject => execute_hash_object_command(&args[3])?,
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
    // Read
    let blob_path = format!("{GIT_OBJECTS_DIR}/{}/{}", &blob_hash[..2], &blob_hash[2..]);
    let blob_data = fs::read(blob_path)?;

    // Decompress
    let mut decoder = ZlibDecoder::new(blob_data.as_slice());
    let mut blob_string = String::new();
    decoder.read_to_string(&mut blob_string)?;

    // Clean
    let Some((_, content_string)) = blob_string.split_once('\0') else {
        return Err(anyhow::anyhow!("Invalid blob data"));
    };

    print!("{}", content_string);
    Ok(())
}

fn execute_hash_object_command(file_path: &str) -> anyhow::Result<()> {
    // Read
    let file_bytes = fs::read(file_path)?;

    // Create a blob content
    let blob_header = format!("blob {}\0", file_bytes.len());
    let blob_content = [blob_header.as_bytes(), file_bytes.as_slice()].concat();

    // Hash
    let mut hasher = Sha1::new();
    hasher.update(blob_content.as_slice());
    let blob_hash = hasher.finalize();
    let blob_hash = hex::encode(blob_hash);

    // Compress
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&blob_content)?;
    let blob_data = encoder.finish()?;

    // Write
    let blob_dir = format!("{GIT_OBJECTS_DIR}/{}", &blob_hash[..2]);
    fs::create_dir(&blob_dir)?;
    let blob_path = format!("{blob_dir}/{}", &blob_hash[2..]);
    fs::write(&blob_path, blob_data)?;

    print!("{blob_hash}");
    Ok(())
}
