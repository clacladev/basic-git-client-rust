use super::tree_line::{Mode, TreeLine};

const HASH_BYTES_LENGTH: usize = 20;

#[derive(Debug, Clone)]
pub struct TreeLines(pub Vec<TreeLine>);

impl TreeLines {
    pub fn new(lines: Vec<TreeLine>) -> Self {
        Self(lines)
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
            lines.push(TreeLine::new(
                Mode::from_string(mode.as_str())?,
                path,
                hash.to_vec(),
            ));

            if loop_remaining_bytes.len() == 0 {
                break;
            }
        }

        Ok(TreeLines::new(lines))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0
            .iter()
            .map(|line| line.to_bytes())
            .flatten()
            .collect()
    }
}
