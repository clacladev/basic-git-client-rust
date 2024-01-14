use self::tree_lines::TreeLines;

pub mod tree_line;
pub mod tree_lines;

pub const GIT_OBJECT_TYPE_BLOB: &str = "blob";
pub const GIT_OBJECT_TYPE_TREE: &str = "tree";

pub enum GitObject {
    Blob(String),
    Tree(TreeLines),
}

impl GitObject {
    pub fn object_type(&self) -> String {
        match self {
            GitObject::Blob(_) => GIT_OBJECT_TYPE_BLOB.to_string(),
            GitObject::Tree(_) => GIT_OBJECT_TYPE_TREE.to_string(),
        }
    }
}

impl GitObject {
    pub fn new(object_type: &str, content_bytes: &[u8]) -> anyhow::Result<Self> {
        match object_type {
            GIT_OBJECT_TYPE_BLOB => {
                let content_string = String::from_utf8_lossy(content_bytes).to_string();
                Ok(GitObject::Blob(content_string))
            }
            GIT_OBJECT_TYPE_TREE => {
                let lines = TreeLines::from_bytes(content_bytes)?;
                Ok(GitObject::Tree(lines))
            }
            _ => Err(anyhow::anyhow!(format!(
                "Invalid object type {}",
                object_type
            ))),
        }
    }
}
