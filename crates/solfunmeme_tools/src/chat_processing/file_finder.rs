use std::path::PathBuf;
use crate::utils;

pub fn find_chat_files(target_path: &PathBuf, limit: Option<usize>) -> Vec<PathBuf> {
    utils::find_files_with_pattern(target_path, "grok-chat", "md", limit)
}

pub fn find_files_with_pattern(target_path: &PathBuf, pattern: &str, file_extension: &str, limit: Option<usize>) -> Vec<PathBuf> {
    utils::find_files_with_pattern(target_path, pattern, file_extension, limit)
}