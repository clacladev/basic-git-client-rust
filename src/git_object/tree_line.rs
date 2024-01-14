const TREE_LINE_MODE_FILE: &str = "100644";
const TREE_LINE_MODE_FOLDER: &str = "40000";

#[derive(Debug, Clone)]
pub enum Mode {
    File,
    Folder,
}

impl Mode {
    // TODO: maybe delete after deleting TreeLines entity
    pub fn from_string(mode_string: &str) -> anyhow::Result<Self> {
        match mode_string {
            TREE_LINE_MODE_FILE => Ok(Mode::File),
            TREE_LINE_MODE_FOLDER => Ok(Mode::Folder),
            _ => Err(anyhow::Error::msg("Invalid mode string")),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Mode::File => TREE_LINE_MODE_FILE.to_string(),
            Mode::Folder => TREE_LINE_MODE_FOLDER.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TreeLine {
    pub mode: Mode,
    pub path: String,
    pub hash: Vec<u8>,
}

impl TreeLine {
    pub fn new(mode: Mode, path: String, hash: Vec<u8>) -> Self {
        Self { mode, path, hash }
    }
}

impl TreeLine {
    pub fn to_bytes(&self) -> Vec<u8> {
        [
            self.mode.to_string().as_bytes(),
            &[b' '],
            self.path.as_bytes(),
            &[b'\0'],
            &self.hash,
        ]
        .concat()
    }
}
