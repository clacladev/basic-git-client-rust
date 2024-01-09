#[derive(Debug)]
pub enum CliCommand {
    Init,
}

impl CliCommand {
    pub fn from_string(string: &str) -> Self {
        match string {
            "init" => CliCommand::Init,
            _ => panic!("Unrecognized command"),
        }
    }
}
