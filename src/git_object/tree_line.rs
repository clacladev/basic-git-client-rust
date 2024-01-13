pub const TREE_LINE_MODE_FILE: &str = "100644";
pub const TREE_LINE_MODE_FOLDER: &str = "40000";

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
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

// impl PartialOrd for TreeLine {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         if self.row == other.row {
//             return Some(self.column.cmp(&other.column));
//         }
//         Some(self.row.cmp(&other.row))
//     }
// }
