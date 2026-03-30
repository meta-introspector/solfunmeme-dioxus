use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod emoji_names;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmojiMatrix {
    pub entries: Vec<EmojiMatrixEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmojiMatrixEntry {
    pub path: String,
    pub emoji_counts: Vec<EmojiCount>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmojiCount {
    pub emoji: String,
    pub name: String,
    pub category: String,
    pub count: u32,
}

// This function will parse the summary_total.txt file and return an EmojiMatrix
pub fn parse_summary_total(file_content: &str) -> EmojiMatrix {
    let _emoji_names = emoji_names::get_emoji_names();
    let mut emoji_counts_map: HashMap<String, u32> = HashMap::new();
    
    let lines: Vec<&str> = file_content.lines().collect();

    // Find the start of the table
    let mut table_start_index = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("Type") {
            table_start_index = i + 2; // Skip the header and the separator line
            break;
        }
    }

    for line in &lines[table_start_index..] {
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        if parts.len() == 4 {
            let count: u32 = parts[1].parse().unwrap_or(0);
            let emoji = parts[3].to_string();
            *emoji_counts_map.entry(emoji).or_insert(0) += count;
        }
    }

    let mut emoji_counts_vec: Vec<EmojiCount> = Vec::new();
    for line in &lines[table_start_index..] {
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        if parts.len() == 4 {
            let name = parts[0].to_string();
            let count: u32 = parts[1].parse().unwrap_or(0);
            let emoji = parts[3].to_string();
            
            // Avoid duplicating entries if the same emoji is used for different types
            if !emoji_counts_vec.iter().any(|ec| ec.emoji == emoji && ec.name == name) {
                emoji_counts_vec.push(EmojiCount {
                    name,
                    emoji,
                    category: parts[2].to_string(),
                    count,
                });
            }
        }
    }

    emoji_counts_vec.sort_by(|a, b| b.count.cmp(&a.count));

    let entry = EmojiMatrixEntry {
        path: "total".to_string(),
        emoji_counts: emoji_counts_vec,
    };

    EmojiMatrix { entries: vec![entry] }
}

// This function will parse the summary_root.txt file and return an EmojiMatrix
pub fn parse_summary_root(file_content: &str) -> EmojiMatrix {
    let emoji_names = emoji_names::get_emoji_names();
    let mut entries = Vec::new();
    
    let sections: Vec<&str> = file_content.split("=== Directory Emoji Summary:").collect();

    for section in sections.iter().skip(1) {
        let lines: Vec<&str> = section.lines().collect();
        let path = lines[0].trim().replace("===", "").trim().to_string();
        let _emoji_counts_map: HashMap<String, u32> = HashMap::new();

        let mut emoji_counts_vec: Vec<EmojiCount> = Vec::new();
        if lines.len() > 1 {
            let emoji_line = lines[1];
            let emoji_parts: Vec<&str> = emoji_line.split('|').collect();
            if emoji_parts.len() > 1 {
                let emojis = emoji_parts[1].trim();
                for emoji_count_str in emojis.split_whitespace() {
                    let parts: Vec<&str> = emoji_count_str.split('×').collect();
                    if parts.len() == 2 {
                        let emoji_and_name_parts: Vec<&str> = parts[0].split('(').collect();
                        let emoji = emoji_and_name_parts[0].to_string();
                        let name = if emoji_and_name_parts.len() > 1 {
                            emoji_and_name_parts[1].trim_end_matches(')').to_string()
                        } else {
                            emoji_names.get(&emoji).map(|(n, _c)| n.clone()).unwrap_or("Unknown".to_string())
                        };
                        let count: u32 = parts[1].parse().unwrap_or(0);
                        
                        let (_name_from_map, category_from_map) = emoji_names.get(&emoji).map(|(n, c)| (n.clone(), c.clone())).unwrap_or(("Unknown".to_string(), "Unknown".to_string()));
                        let category = if category_from_map == "Rust Core" {
                            emoji_names::get_rust_core_sub_category(&name).to_string()
                        } else {
                            category_from_map
                        };

                        if !emoji_counts_vec.iter().any(|ec| ec.emoji == emoji && ec.name == name) {
                            emoji_counts_vec.push(EmojiCount {
                                name,
                                emoji,
                                category,
                                count,
                            });
                        }
                    }
                }
            }
        }

        emoji_counts_vec.sort_by(|a, b| b.count.cmp(&a.count));

        entries.push(EmojiMatrixEntry {
            path,
            emoji_counts: emoji_counts_vec,
        });
    }

    EmojiMatrix { entries }
}

pub fn rollup_emoji_matrix(matrix: EmojiMatrix) -> EmojiMatrix {
    let emoji_names = emoji_names::get_emoji_names();
    let mut rolled_up_entries: HashMap<String, HashMap<String, u32>> = HashMap::new();

    log::info!("Rolling up emoji matrix with {} entries", matrix.entries.len());

    for entry in matrix.entries {
        let path_parts: Vec<&str> = entry.path.split('/').collect();
        let mut current_path = String::new();

        for (i, part) in path_parts.iter().enumerate() {
            if i > 0 {
                current_path.push_str("/");
            }
            current_path.push_str(part);

            let entry_counts = rolled_up_entries.entry(current_path.clone()).or_default();
            for emoji_count in entry.emoji_counts.iter() {
                *entry_counts.entry(emoji_count.emoji.clone()).or_insert(0) += emoji_count.count;
            }
        }
    }

    let mut entries = Vec::new();
    for (path, emoji_counts_map) in rolled_up_entries {
        let mut emoji_counts_vec: Vec<EmojiCount> = emoji_counts_map
            .into_iter()
            .map(|(emoji, count)| {
                // We need to find the name for the emoji. 
                // This is not ideal, as we lose the original name from the file.
                // For now, we will use the emoji_names map.
                let (name_from_map, category_from_map) = emoji_names.get(&emoji).map(|(n, c)| (n.clone(), c.clone())).unwrap_or(("Unknown".to_string(), "Unknown".to_string()));
                let name = name_from_map;
                let category = if category_from_map == "Rust Core" {
                    emoji_names::get_rust_core_sub_category(&name).to_string()
                } else {
                    category_from_map
                };
                EmojiCount {
                    name,
                    emoji,
                    category,
                    count,
                }
            })
            .collect();

        emoji_counts_vec.sort_by(|a, b| b.count.cmp(&a.count));

        entries.push(EmojiMatrixEntry {
            path,
            emoji_counts: emoji_counts_vec,
        });
    }
    log::info!("Rolled up emoji matrix to {} entries", entries.len());
    EmojiMatrix { entries }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_summary_total() {
        let content = r#"""
=== Total Emoji Summary Table ===
Type                 | Count    | Category           | Emoji
---------------------|----------|--------------------|--------
_                    | 137      | Rust Core          | ⬜
angle_bracketed      | 959      | Rust Core          | ⟨⟩
args                 | 5809     | Rust Core          | 📢
"""#;
        let matrix = parse_summary_total(content);
        assert_eq!(matrix.entries.len(), 1);
        let entry = &matrix.entries[0];
        assert_eq!(entry.path, "total");
        assert_eq!(entry.emoji_counts.len(), 3);
        assert_eq!(entry.emoji_counts[0].emoji, "📢");
        assert_eq!(entry.emoji_counts[0].count, 5809);
        assert_eq!(entry.emoji_counts[0].name, "Loudspeaker");
    }

    #[test]
    fn test_parse_summary_root() {
        let content = r#"""
=== Directory Emoji Summary: src/models ===
⬜(White Large Square)×10 🔗(Link)×5
=== Directory Emoji Summary: src/views ===
💡(Light Bulb)×8 🎨(Artist Palette)×3
"""#;
        let matrix = parse_summary_root(content);
        assert_eq!(matrix.entries.len(), 2);

        let entry1 = &matrix.entries[0];
        assert_eq!(entry1.path, "src/models");
        assert_eq!(entry1.emoji_counts.len(), 2);
        assert_eq!(entry1.emoji_counts[0].emoji, "⬜");
        assert_eq!(entry1.emoji_counts[0].count, 10);
        assert_eq!(entry1.emoji_counts[0].name, "White Large Square");

        let entry2 = &matrix.entries[1];
        assert_eq!(entry2.path, "src/views");
        assert_eq!(entry2.emoji_counts.len(), 2);
        assert_eq!(entry2.emoji_counts[0].emoji, "💡");
        assert_eq!(entry2.emoji_counts[0].count, 8);
        assert_eq!(entry2.emoji_counts[0].name, "Light Bulb");
    }

    #[test]
    fn test_rollup_emoji_matrix() {
        let entry1 = EmojiMatrixEntry {
            path: "src/models".to_string(),
            emoji_counts: vec![
                EmojiCount { emoji: "⬜".to_string(), name: "White Large Square".to_string(), count: 10, category: Some("shapes".to_string()) },
                EmojiCount { emoji: "🔗".to_string(), name: "Link".to_string(), count: 5, category: Some("symbols".to_string()) },
            ],
        };
        let entry2 = EmojiMatrixEntry {
            path: "src/views".to_string(),
            emoji_counts: vec![
                EmojiCount { emoji: "💡".to_string(), name: "Light Bulb".to_string(), count: 8 },
                EmojiCount { emoji: "🎨".to_string(), name: "Artist Palette".to_string(), count: 3 },
            ],
        };
        let entry3 = EmojiMatrixEntry {
            path: "src/models/sub".to_string(),
            emoji_counts: vec![
                EmojiCount { 
                    emoji: "⬜".to_string(), 
                    name: "White Large Square".to_string(), 
                    count: 2, 
                    category: Some("shapes".to_string()),
                },
            ],
        };

        let matrix = EmojiMatrix { entries: vec![entry1, entry2, entry3] };
        let rolled_up_matrix = rollup_emoji_matrix(matrix);

        assert_eq!(rolled_up_matrix.entries.len(), 3);

        // Check "src" entry
        let src_entry = rolled_up_matrix.entries.iter().find(|e| e.path == "src").unwrap();
        assert_eq!(src_entry.emoji_counts.len(), 4);
        assert_eq!(src_entry.emoji_counts[0].emoji, "⬜"); // 10 + 2 = 12
        assert_eq!(src_entry.emoji_counts[0].count, 12);

        // Check "src/models" entry
        let models_entry = rolled_up_matrix.entries.iter().find(|e| e.path == "src/models").unwrap();
        assert_eq!(models_entry.emoji_counts.len(), 2);
        assert_eq!(models_entry.emoji_counts[0].emoji, "⬜"); // 10 + 2 = 12
        assert_eq!(models_entry.emoji_counts[0].count, 12);

        // Check "src/views" entry
        let views_entry = rolled_up_matrix.entries.iter().find(|e| e.path == "src/views").unwrap();
        assert_eq!(views_entry.emoji_counts.len(), 2);
        assert_eq!(views_entry.emoji_counts[0].emoji, "💡");
        assert_eq!(views_entry.emoji_counts[0].count, 8);
    }
}