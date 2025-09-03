use dioxus::prelude::*;
use reqwest;
use emoji_matrix_lib::{parse_summary_total, parse_summary_root, rollup_emoji_matrix, EmojiMatrix};

#[tokio::test]
async fn emoji_matrix_live_server_test() {
    let base_url = "http://192.168.68.54:8080";

    // Fetch summary_total.txt
    let total_url = format!("{}/reports/summary_total.txt", base_url);
    let total_res = reqwest::get(&total_url).await.expect("Failed to fetch summary_total.txt");
    let total_text = total_res.text().await.expect("Failed to get text from summary_total.txt");
    let mut combined_entries = parse_summary_total(&total_text).entries;

    // Fetch summary_root.txt
    let root_url = format!("{}/reports/summary_root.txt", base_url);
    let root_res = reqwest::get(&root_url).await.expect("Failed to fetch summary_root.txt");
    let root_text = root_res.text().await.expect("Failed to get text from summary_root.txt");
    combined_entries.extend(parse_summary_root(&root_text).entries);

    let final_matrix = rollup_emoji_matrix(EmojiMatrix { entries: combined_entries });

    // Assertions - you can add more specific assertions here based on expected data
    assert!(!final_matrix.entries.is_empty(), "Emoji matrix should not be empty");

    // Example: Check if 'total' entry exists and has some emojis
    let total_entry = final_matrix.entries.iter().find(|e| e.path == "total");
    assert!(total_entry.is_some(), "'total' entry should exist");
    assert!(!total_entry.unwrap().emoji_counts.is_empty(), "'total' entry should have emoji counts");

    // Example: Check for a specific directory entry
    let src_core_entry = final_matrix.entries.iter().find(|e| e.path == "src/core");
    assert!(src_core_entry.is_some(), "'src/core' entry should exist");
    // Add more specific assertions for src/core if needed
}