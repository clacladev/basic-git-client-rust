use self::tree_lines::TreeLines;

pub mod tree_line;
pub mod tree_lines;

pub const GIT_OBJECT_TYPE_BLOB: &str = "blob";
pub const GIT_OBJECT_TYPE_TREE: &str = "tree";
pub const GIT_OBJECT_TYPE_COMMIT: &str = "commit";

pub enum GitObject {
    Blob(String),
    Tree(TreeLines),
    Commit(String),
}

impl GitObject {
    pub fn object_type(&self) -> String {
        match self {
            GitObject::Blob(_) => GIT_OBJECT_TYPE_BLOB.to_string(),
            GitObject::Tree(_) => GIT_OBJECT_TYPE_TREE.to_string(),
            GitObject::Commit(_) => GIT_OBJECT_TYPE_COMMIT.to_string(),
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
            GIT_OBJECT_TYPE_COMMIT => {
                let content_string = String::from_utf8_lossy(content_bytes).to_string();
                Ok(GitObject::Commit(content_string))
            }
            _ => Err(anyhow::anyhow!(format!(
                "Invalid object type {}",
                object_type
            ))),
        }
    }

    pub fn new_commit(tree_hash: &str, parent_commit_hash: &str, message: &str) -> Self {
        let mut commit_message = format!("tree {tree_hash}\n");
        commit_message.push_str(format!("parent {parent_commit_hash}\n").as_str());
        commit_message.push_str("author Johnny Bravo <johnny@bravo.com> 1493170892 -0500\n");
        commit_message.push_str("committer Johnny Bravo <johnny@bravo.com> 1493170892 -0500\n\n");
        commit_message.push_str(format!("{message}\n").as_str());
        GitObject::Commit(commit_message)
    }
}
