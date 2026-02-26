use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub status: char, // M, A, D, R, C
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoStatus {
    pub name: String,
    pub path: PathBuf,
    pub branch: String,
    pub is_dirty: bool,
    pub staged: Vec<FileChange>,
    pub unstaged: Vec<FileChange>,
    pub untracked: Vec<String>,
    pub ahead: u32,
    pub behind: u32,
    pub last_commit_message: String,
    pub last_commit_time: String,      // relative time like "5m ago", "2h ago"
    pub last_commit_epoch: i64,        // unix timestamp for sorting (0 if unknown)
    #[serde(skip)]
    pub stale: bool, // true = cached data not yet refreshed
}

impl RepoStatus {
    pub fn total_changes(&self) -> usize {
        self.staged.len() + self.unstaged.len() + self.untracked.len()
    }
}
