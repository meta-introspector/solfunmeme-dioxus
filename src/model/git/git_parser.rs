//#[cfg(feature = "git2")]
//use git2::{Repository, ObjectType, Oid};

use syn::{File, parse_str};
use serde_json::Value;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitParserError {
    #[error("Git error: {0}")]
    #[cfg(feature = "git2")]
    Git(#[from] git2::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct GitParser {
    repo: Repository,
}

impl GitParser {
    pub fn new(repo_path: &str) -> Result<Self, GitParserError> {
        let repo = Repository::open(repo_path)?;
        Ok(Self { repo })
    }

    pub fn get_file_at_commit(&self, commit_id: &str, file_path: &str) -> Result<String, GitParserError> {
        let oid = Oid::from_str(commit_id)?;
        let commit = self.repo.find_commit(oid)?;
        let tree = commit.tree()?;
        let entry = tree.get_path(Path::new(file_path))?;
        let blob = entry.to_object(&self.repo)?;
        
        if let Some(blob) = blob.as_blob() {
            Ok(String::from_utf8_lossy(blob.content()).to_string())
        } else {
            Err(GitParserError::Parse("Object is not a blob".to_string()))
        }
    }

    pub fn parse_rust_file(&self, content: &str) -> Result<Value, GitParserError> {
        let syntax = parse_str::<File>(content)
            .map_err(|e| GitParserError::Parse(e.to_string()))?;
        
        // Convert the AST to JSON using syn-serde
        let json = serde_json::to_value(&syntax)
            .map_err(|e| GitParserError::Parse(e.to_string()))?;
        
        Ok(json)
    }

    pub fn get_all_commits(&self) -> Result<Vec<String>, GitParserError> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        
        let mut commits = Vec::new();
        for oid in revwalk {
            let oid = oid?;
            commits.push(oid.to_string());
        }
        
        Ok(commits)
    }

    pub fn get_file_history(&self, file_path: &str) -> Result<Vec<(String, String)>, GitParserError> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        
        let mut history = Vec::new();
        for oid in revwalk {
            let oid = oid?;
            if let Ok(content) = self.get_file_at_commit(&oid.to_string(), file_path) {
                history.push((oid.to_string(), content));
            }
        }
        
        Ok(history)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_parser() {
        // Add tests here
    }
} 