#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Read;

use cli_commands::CliCommand;
use flate2::read::ZlibDecoder;

mod cli_commands;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = CliCommand::from_string(&args[1]);

    match command {
        CliCommand::Init => execute_init_command()?,
        CliCommand::CatFile => execute_cat_file_command(&args[3])?,
    }

    Ok(())
}

fn execute_init_command() -> anyhow::Result<()> {
    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/master\n")?;
    println!("Initialized git directory");
    Ok(())
}

fn execute_cat_file_command(blob_hash: &str) -> anyhow::Result<()> {
    let blob_path = format!(".git/objects/{}/{}", &blob_hash[..2], &blob_hash[2..]);
    let blob_data = fs::read(blob_path)?;

    let mut decoder = ZlibDecoder::new(blob_data.as_slice());
    let mut blob_string = String::new();
    decoder.read_to_string(&mut blob_string)?;

    let Some(blob_string) = blob_string.split_once('\0') else {
        return Err(anyhow::anyhow!("Invalid blob data"));
    };
    let blob_string = blob_string.1;
    print!("{}", blob_string);

    Ok(())
}
