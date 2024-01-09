#[derive(Debug)]
pub enum CliCommand {
    Init,
    CatFile,
}

impl CliCommand {
    pub fn from_string(string: &str) -> Self {
        match string {
            "init" => CliCommand::Init,
            "cat-file" => CliCommand::CatFile,
            _ => panic!("Unrecognized command"),
        }
    }
}
