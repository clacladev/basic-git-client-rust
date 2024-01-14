pub const TREE_LINE_MODE_FILE: &str = "100644";
pub const TREE_LINE_MODE_FOLDER: &str = "40000";

#[derive(Debug, Clone)]
pub struct TreeLine {
    pub mode: String,
    pub path: String,
    pub hash: Vec<u8>,
}

impl TreeLine {
    pub fn new(mode: String, path: String, hash: Vec<u8>) -> Self {
        Self { mode, path, hash }
    }
}

impl TreeLine {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend_from_slice(self.mode.as_bytes());
        bytes.push(b' ');
        bytes.extend_from_slice(self.path.as_bytes());
        bytes.push(b'\0');
        bytes.extend_from_slice(&self.hash);
        bytes
    }
}
