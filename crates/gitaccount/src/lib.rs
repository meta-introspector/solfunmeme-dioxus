use borsh::{BorshSerialize, BorshDeserialize};

/// Unique identifier for a repo (could be a Pubkey, hash, or UUID)
pub type RepoId = [u8; 32];

/// Unique identifier for a git object (blob/tree/commit)
pub type ObjectId = [u8; 32];

/// Unique identifier for a branch (could be a name hash or UUID)
pub type BranchId = [u8; 32];

/// Core repo metadata
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Repo {
    pub id: RepoId,
    pub name: String,
    pub description: String,
    pub owner: [u8; 32], // e.g., a pubkey
    pub root_tree: ObjectId,
    pub created_at: u64,
}

/// A branch in a repo
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Branch {
    pub id: BranchId,
    pub repo: RepoId,
    pub name: String,
    pub head_commit: ObjectId,
    pub created_by: [u8; 32],
    pub created_at: u64,
}

/// A git object (blob, tree, commit)
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum GitObject {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Blob {
    pub id: ObjectId,
    pub data: Vec<u8>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TreeEntry {
    pub name: String,
    pub object: ObjectId,
    pub is_tree: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Tree {
    pub id: ObjectId,
    pub entries: Vec<TreeEntry>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct Commit {
    pub id: ObjectId,
    pub tree: ObjectId,
    pub parents: Vec<ObjectId>,
    pub author: [u8; 32],
    pub message: String,
    pub timestamp: u64,
}