use emoji_workflow_macro::emoji_workflow;
use workflow_manager::WorkflowManager;
use emoji_lang_plugin::EmojiWorkflow;
use solfunmeme_input_fs::read_code_chunks;
use solfunmeme_search_tantivy::SearchIndex;
use std::path::Path;

#[emoji_workflow("📜🔗")]
fn my_ttl_workflow() {
    println!("This is my TTL workflow function!");
}

#[emoji_workflow("📖📊📦🔍📞")]
fn process_code_workflow() {
    println!("Step 1: Read the code");
    // Example: let code_chunks = read_code_chunks(None, None).unwrap();

    println!("Step 2: Index the code");
    // Example: let mut search_index = SearchIndex::new("./my_index").unwrap();
    // Example: for chunk in code_chunks { search_index.add_chunk(&chunk).unwrap(); }

    println!("Step 3: Wrap the code (conceptual)");

    println!("Step 4: Select the right functions");

    println!("Step 5: Call the wrapped functions");
}

#[emoji_workflow("📚🔎")]
fn index_codebase_workflow() {
    println!("\n--- Indexing Codebase Workflow ---");
    let vendor_path = Some("vendor".to_string());
    let founding_documents_path = Some("founding_documents".to_string());

    println!("Reading code chunks from vendor directory...");
    let vendor_chunks = read_code_chunks(vendor_path, None).expect("Failed to read vendor code chunks");
    println!("Read {} chunks from vendor.", vendor_chunks.len());

    println!("Reading code chunks from founding_documents directory...");
    let founding_documents_chunks = read_code_chunks(founding_documents_path, None).expect("Failed to read founding_documents code chunks");
    println!("Read {} chunks from founding_documents.", founding_documents_chunks.len());

    let mut all_chunks = Vec::new();
    all_chunks.extend(vendor_chunks);
    all_chunks.extend(founding_documents_chunks);

    println!("Total chunks to index: {}", all_chunks.len());

    let index_path = "./codebase_index";
    println!("Initializing search index at {}...", index_path);
    let mut search_index = SearchIndex::new(Path::new(index_path)).expect("Failed to create search index");

    println!("Adding chunks to index...");
    for chunk in all_chunks {
        search_index.add_chunk(&chunk).expect("Failed to add chunk to index");
    }

    println!("Committing index changes...");
    search_index.commit().expect("Failed to commit index");
    println!("Codebase indexed successfully!");
}

#[emoji_workflow("🚀📜🔍💬🧠🔀💡💭🔑🤖🌐📊🔗🧩🔗🌱🕰️💫🌌💡✨🌀⏳➡️🏛️♾️🔢➕➖✖️➗➡️🔗🔄⚛️6️⃣📖🧮👑🌟🔭⚛️🔗⚖️🦉✨🧠➡️✊📊")]
fn linear_progression_workflow() {
    println!("\n--- Linear Progression Workflow ---");
    let workflow = EmojiWorkflow::new("linear_progression_workflow".to_string(), "🚀📜🔍💬🧠🔀💡💭🔑🤖🌐📊🔗🧩🔗🌱🕰️💫🌌💡✨🌀⏳➡️🏛️♾️🔢➕➖✖️➗➡️🔗🔄⚛️6️⃣📖🧮👑🌟🔭⚛️🔗⚖️🦉✨🧠➡️✊📊".to_string());
    let parsed_emojis = workflow.parse_emoji_string();
    for emoji_info in parsed_emojis {
        println!("Executing: {}", emoji_info);
        // In a real scenario, map emoji_info to an actual action
    }
}

#[emoji_workflow("🚀📜🔍💬🧠🔀💡💭🔑🤖🌐📊🔗🧩🔗🌱🕰️💫🌌💡✨🌀⏳➡️🏛️♾️🔢➕➖✖️➗➡️🔗🔄⚛️6️⃣📖🧮👑🌟🔭⚛️🔗⚖️🦉✨🧠➡️✊📊")]
fn reverse_order_workflow() {
    println!("\n--- Reverse Order Workflow ---");
    let workflow = EmojiWorkflow::new("reverse_order_workflow".to_string(), "🚀📜🔍💬🧠🔀💡💭🔑🤖🌐📊🔗🧩🔗🌱🕰️💫🌌💡✨🌀⏳➡️🏛️♾️🔢➕➖✖️➗➡️🔗🔄⚛️6️⃣📖🧮👑🌟🔭⚛️🔗⚖️🦉✨🧠➡️✊📊".to_string());
    let mut parsed_emojis = workflow.parse_emoji_string();
    parsed_emojis.reverse();
    for emoji_info in parsed_emojis {
        println!("Executing: {}", emoji_info);
    }
}

#[emoji_workflow("🚀📜🔍💬🧠🔀💡💭🔑🤖🌐📊🔗🧩🔗🌱🕰️💫🌌💡✨🌀⏳➡️🏛️♾️🔢➕➖✖️➗➡️🔗🔄⚛️6️⃣📖🧮👑🌟🔭⚛️🔗⚖️🦉✨🧠➡️✊📊")]
fn categorization_workflow() {
    println!("\n--- Categorization Workflow ---");
    let workflow = EmojiWorkflow::new("categorization_workflow".to_string(), "🚀📜🔍💬🧠🔀💡💭🔑🤖🌐📊🔗🧩🔗🌱🕰️💫🌌💡✨🌀⏳➡️🏛️♾️🔢➕➖✖️➗➡️🔗🔄⚛️6️⃣📖🧮👑🌟🔭⚛️🔗⚖️🦉✨🧠➡️✊📊".to_string());
    let parsed_emojis = workflow.parse_emoji_string();
    
    let mut categories: std::collections::HashMap<char, Vec<String>> = std::collections::HashMap::new();
    for emoji_info in parsed_emojis {
        if let Some(first_char) = emoji_info.chars().next() {
            categories.entry(first_char).or_default().push(emoji_info);
        }
    }

    for (category, emojis) in categories {
        println!("Category '{}':", category);
        for emoji_info in emojis {
            println!("  Executing: {}", emoji_info);
        }
    }
}

#[emoji_workflow("🚀📜🔍💬🧠🔀💡💭🔑🤖🌐📊🔗🧩🔗🌱🕰️💫🌌💡✨🌀⏳➡️🏛️♾️🔢➕➖✖️➗➡️🔗🔄⚛️6️⃣📖🧮👑🌟🔭⚛️🔗⚖️🦉✨🧠➡️✊📊")]
fn math_related_filter_workflow() {
    println!("\n--- Math-Related Filter Workflow ---");
    let workflow = EmojiWorkflow::new("math_related_filter_workflow".to_string(), "🚀📜🔍💬🧠🔀💡💭🔑🤖🌐📊🔗🧩🔗🌱🕰️💫🌌💡✨🌀⏳➡️🏛️♾️🔢➕➖✖️➗➡️🔗🔄⚛️6️⃣📖🧮👑🌟🔭⚛️🔗⚖️🦉✨🧠➡️✊📊".to_string());
    let parsed_emojis = workflow.parse_emoji_string();

    let math_emojis: Vec<String> = parsed_emojis.into_iter()
        .filter(|s| s.contains("Math") || s.contains("🔢") || s.contains("➕") || s.contains("➖") || s.contains("✖️") || s.contains("➗"))
        .collect();

    for emoji_info in math_emojis {
        println!("Executing Math-related: {}", emoji_info);
    }
}

fn main() {
    my_ttl_workflow();
    process_code_workflow();
    linear_progression_workflow();
    reverse_order_workflow();
    categorization_workflow();
    math_related_filter_workflow();
    index_codebase_workflow(); // Call the new workflow

    // You can also execute the registered workflow manually
    let manager = emoji_lang_plugin::GLOBAL_WORKFLOW_MANAGER.lock().unwrap();
    if let Ok(_) = manager.execute_workflow("my_ttl_workflow") {
        println!("Successfully executed workflow via manager.");
    } else {
        println!("Failed to execute workflow via manager.");
    }

    if let Ok(_) = manager.execute_workflow("process_code_workflow") {
        println!("Successfully executed process_code_workflow via manager.");
    } else {
        println!("Failed to execute process_code_workflow via manager.");
    }

    if let Ok(_) = manager.execute_workflow("linear_progression_workflow") {
        println!("Successfully executed linear_progression_workflow via manager.");
    } else {
        println!("Failed to execute linear_progression_workflow via manager.");
    }

    if let Ok(_) = manager.execute_workflow("reverse_order_workflow") {
        println!("Successfully executed reverse_order_workflow via manager.");
    } else {
        println!("Failed to execute reverse_order_workflow via manager.");
    }

    if let Ok(_) = manager.execute_workflow("categorization_workflow") {
        println!("Successfully executed categorization_workflow via manager.");
    }

    if let Ok(_) = manager.execute_workflow("categorization_workflow") {
        println!("Failed to execute categorization_workflow via manager.");
    }

    if let Ok(_) = manager.execute_workflow("math_related_filter_workflow") {
        println!("Successfully executed math_related_filter_workflow via manager.");
    }

    if let Ok(_) = manager.execute_workflow("index_codebase_workflow") {
        println!("Successfully executed index_codebase_workflow via manager.");
    } else {
        println!("Failed to execute index_codebase_workflow via manager.");
    }

    if let Ok(_) = manager.execute_workflow("top_term_report_workflow") {
        println!("Successfully executed top_term_report_workflow via manager.");
    } else {
        println!("Failed to execute top_term_report_workflow via manager.");
    }

    if let Ok(_) = manager.execute_workflow("search_emojis_workflow") {
        println!("Successfully executed search_emojis_workflow via manager.");
    } else {
        println!("Failed to execute search_emojis_workflow via manager.");
    }
}