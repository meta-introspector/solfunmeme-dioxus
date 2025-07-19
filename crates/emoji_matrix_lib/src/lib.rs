use rust_embed::RustEmbed;
use log::error;
pub use core_data_lib::{EmojiMatrix, EmojiMatrixEntry, EmojiCount, parse_summary_total as core_parse_summary_total, parse_summary_root as core_parse_summary_root, rollup_emoji_matrix as core_rollup_emoji_matrix};

#[derive(RustEmbed)]
#[folder = "reports/"]
struct EmbeddedReports;

pub fn parse_summary_total() -> EmojiMatrix {
    let file_content = if let Some(file) = EmbeddedReports::get("summary_total.txt") {
        match String::from_utf8(file.data.into_owned()) {
            Ok(content) => content,
            Err(e) => {
                error!("Failed to convert summary_total.txt to UTF-8: {}", e);
                String::new()
            }
        }
    } else {
        error!("Failed to find summary_total.txt in embedded reports.");
        String::new()
    };
    core_parse_summary_total(&file_content)
}

pub fn parse_summary_root() -> EmojiMatrix {
    let file_content = if let Some(file) = EmbeddedReports::get("summary_root.txt") {
        match String::from_utf8(file.data.into_owned()) {
            Ok(content) => content,
            Err(e) => {
                error!("Failed to convert summary_root.txt to UTF-8: {}", e);
                String::new()
            }
        }
    } else {
        error!("Failed to find summary_root.txt in embedded reports.");
        String::new()
    };
    core_parse_summary_root(&file_content)
}

pub fn rollup_emoji_matrix(matrix: EmojiMatrix) -> EmojiMatrix {
    core_rollup_emoji_matrix(matrix)
}
