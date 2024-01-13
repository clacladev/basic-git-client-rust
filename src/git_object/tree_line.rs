const HASH_BYTES_LENGTH: usize = 20;

pub const TREE_LINE_MODE_FILE: &str = "100644";
pub const TREE_LINE_MODE_FOLDER: &str = "40000";

#[derive(Debug, Clone)]
pub struct TreeLine {
    pub mode: String,
    pub path: String,
    pub hash: String,
}

impl TreeLine {
    pub fn new(mode: &str, path: &str, hash: &str) -> Self {
        Self {
            mode: mode.to_string(),
            path: path.to_string(),
            hash: hash.to_string(),
        }
    }
}

impl TreeLine {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend_from_slice(self.mode.as_bytes());
        bytes.push(b' ');
        bytes.extend_from_slice(self.path.as_bytes());
        bytes.push(b'\0');
        bytes.extend_from_slice(hex::decode(self.hash.as_str()).unwrap().as_slice());

        bytes
    }
}

#[derive(Debug, Clone)]
pub struct TreeLines(pub Vec<TreeLine>);

impl TreeLines {
    pub fn new(lines: &[TreeLine]) -> Self {
        Self(lines.to_vec())
    }
}

impl TreeLines {
    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let mut lines: Vec<TreeLine> = vec![];
        let mut loop_remaining_bytes: &[u8] = bytes;

        loop {
            let Some(space_index) = loop_remaining_bytes.iter().position(|&b| b == b' ') else {
                return Err(anyhow::anyhow!("Invalid tree line"));
            };
            let mode = &loop_remaining_bytes[..space_index];
            loop_remaining_bytes = &loop_remaining_bytes[(space_index + 1)..];

            let Some(null_index) = loop_remaining_bytes.iter().position(|&b| b == b'\0') else {
                return Err(anyhow::anyhow!("Invalid tree line"));
            };
            let path = &loop_remaining_bytes[..null_index];
            loop_remaining_bytes = &loop_remaining_bytes[(null_index + 1)..];

            let hash = &loop_remaining_bytes[..HASH_BYTES_LENGTH];
            loop_remaining_bytes = &loop_remaining_bytes[HASH_BYTES_LENGTH..];

            let mode = String::from_utf8_lossy(mode).to_string();
            let path = String::from_utf8_lossy(path).to_string();
            let hash = hex::encode(hash);

            lines.push(TreeLine::new(mode.as_str(), path.as_str(), hash.as_str()));

            if loop_remaining_bytes.len() == 0 {
                break;
            }
        }

        Ok(TreeLines::new(&lines))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        for line in &self.0 {
            bytes.extend_from_slice(line.to_bytes().as_slice());
        }

        bytes
    }
}
