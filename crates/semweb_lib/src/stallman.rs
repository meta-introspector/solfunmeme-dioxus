use async_trait::async_trait;
use sophia::inmem::graph::FastGraph;
use crate::SemWebResult;

#[async_trait]
pub trait Stallman {
    /// Vibe: Check for free software compliance
    fn check_fsf_compliance(&self, graph: &FastGraph) -> SemWebResult<FSFCompliance>;
    /// Vibe: List Emacs-like extensibility features
    fn list_emacs_features(&self, graph: &FastGraph) -> SemWebResult<Vec<String>>;
}

#[derive(Debug, Clone)]
pub struct FSFCompliance {
    pub is_free: bool,
    pub license: String,
    pub notes: String,
}

pub struct StallmanImpl;

#[async_trait]
impl Stallman for StallmanImpl {
    fn check_fsf_compliance(&self, _graph: &FastGraph) -> SemWebResult<FSFCompliance> {
        Ok(FSFCompliance { is_free: true, license: "GPLv3".to_string(), notes: "Stub: free software".to_string() })
    }
    fn list_emacs_features(&self, _graph: &FastGraph) -> SemWebResult<Vec<String>> {
        Ok(vec!["Lisp extensibility".to_string(), "Keybindings".to_string()])
    }
} 