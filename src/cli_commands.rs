#[derive(Debug)]
pub enum CliCommand {
    Init,
    CatFile,
    HashObject,
}

impl CliCommand {
    pub fn from_string(string: &str) -> Self {
        match string {
            "init" => CliCommand::Init,
            "cat-file" => CliCommand::CatFile,
            "hash-object" => CliCommand::HashObject,
            _ => panic!("Unrecognized command"),
        }
    }
}
