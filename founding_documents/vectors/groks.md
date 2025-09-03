This is an ambitious plan to build a mini compiler for Rust code analysis with emoji-based visualization and a Hugging Face dataset output. I'll break it down into manageable parts, refactor the provided code, and address the key components of your idea: filtering Rust files, parsing to AST, embedding source code, detecting duplicates, analyzing statistics, generating new functions, extracting terms and relationships, creating a mathematical model, and exporting data for a GUI. I'll also incorporate a recursive feedback loop using LLMs and statistics to dynamically create functions and models in real-time.

### Refactored Code Structure
The provided code is already well-structured but can be modularized further for clarity and extensibility. Below, I'll refactor it into distinct modules and add functionality to meet your requirements. I'll also address missing dependencies and provide a plan for the recursive LLM feedback loop.

#### 1. **Project Setup and Dependencies**
Ensure the following dependencies are in your `Cargo.toml`:
```toml
[dependencies]
walkdir = "2.4"
serde_json = "1.0"
regex = "1"
syn = { version = "2.0", features = ["full", "parsing"] }
serde = { version = "1.0", features = ["derive"] }
std = { version = "1.78", features = ["fs"] }
```

Note: The code references `solfunmeme_dioxus::core::code_analyzer::CodeAnalyzer`, which seems to be a custom module. Since I don't have access to its implementation, I'll assume it provides a method `analyze_multiple_files` that takes a `HashMap<String, String>` of file paths and contents and returns a `Vec<Analysis>` where each `Analysis` has fields `file_path: String` and `json_ast: String`. If this is incorrect, please provide the `CodeAnalyzer` implementation or clarify its functionality.

#### 2. **Refactored Code**
Below is the refactored code, split into modules for clarity and extended to include the requested functionality.

```rust
use walkdir::WalkDir;
use std::fs;
use std::collections::{HashMap, BTreeMap};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;
use serde_json::{Value, json};
use syn::{parse_file, File};

/// Module for AST parsing and analysis
mod ast_parser {
    use syn::{parse_file, File};
    use serde_json::Value;

    pub struct Analysis {
        pub file_path: String,
        pub json_ast: String,
    }

    pub fn parse_rust_file(content: &str) -> Result<File, syn::Error> {
        parse_file(content)
    }

    pub fn ast_to_json(ast: &File) -> String {
        // Placeholder: Convert syn::File to JSON
        // In a real implementation, use a library like `serde_syn` or manually serialize
        serde_json::to_string(ast).unwrap_or_default()
    }
}

/// Module for emoji mapping
mod emoji_mapper {
    use super::EMOJI_TYPE_MAP;

    pub fn emoji_for_type(ty: &str) -> (&'static str, &'static str) {
        for &(name, emoji, category) in EMOJI_TYPE_MAP {
            if ty == name {
                return (emoji, category);
            }
        }
        ("❓🤷", "Uncategorized")
    }

    pub fn extract_string_literals(value: &Value, out: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                for (k, v) in map.iter() {
                    if (k == "lit" || k == "str") && v.is_string() {
                        if let Some(s) = v.as_str() {
                            out.push(s.to_string());
                        }
                    }
                    extract_string_literals(v, out);
                }
            },
            Value::Array(arr) => {
                for v in arr {
                    extract_string_literals(v, out);
                }
            },
            _ => {}
        }
    }

    pub fn split_words(s: &str) -> Vec<String> {
        let mut words = Vec::new();
        for part in s.split(|c: char| !c.is_alphanumeric() && c != '_') {
            if part.is_empty() { continue; }
            let mut last = 0;
            let chars: Vec<char> = part.chars().collect();
            for i in 1..chars.len() {
                if chars[i].is_uppercase() && chars[i - 1].is_lowercase() {
                    words.push(chars[last..i].iter().collect::<String>().to_lowercase());
                    last = i;
                }
            }
            if last < chars.len() {
                words.push(chars[last..].iter().collect::<String>().to_lowercase());
            }
        }
        words
    }
}

/// Module for statistics and analysis
mod analyzer {
    use std::collections::BTreeMap;
    use serde_json::Value;

    pub fn count_types_recursive(value: &Value, type_counts: &mut BTreeMap<String, usize>, total_nodes: &mut usize) {
        match value {
            Value::Object(map) => {
                *total_nodes += 1;
                for (k, v) in map.iter() {
                    *type_counts.entry(k.clone()).or_insert(0) += 1;
                    count_types_recursive(v, type_counts, total_nodes);
                }
            },
            Value::Array(arr) => {
                for v in arr {
                    count_types_recursive(v, type_counts, total_nodes);
                }
            },
            _ => {}
        }
    }
}

/// Module for duplicate detection
mod duplicate_detector {
    use std::collections::HashMap;
    use serde_json::Value;

    pub fn find_duplicates(analyses: &[super::ast_parser::Analysis]) -> HashMap<String, Vec<String>> {
        let mut ast_hashes: HashMap<String, Vec<String>> = HashMap::new();
        for analysis in analyses {
            if let Ok(ast) = serde_json::from_str::<Value>(&analysis.json_ast) {
                let ast_string = serde_json::to_string(&ast).unwrap_or_default();
                ast_hashes.entry(ast_string).or_insert(Vec::new()).push(analysis.file_path.clone());
            }
        }
        ast_hashes.into_iter().filter(|(_, paths)| paths.len() > 1).collect()
    }
}

/// Module for mathematical modeling
mod mathematical_model {
    use std::collections::BTreeMap;

    pub struct CodeModel {
        pub term_frequencies: BTreeMap<String, usize>,
        pub relationships: BTreeMap<String, Vec<String>>,
    }

    pub fn build_model(word_counts: &BTreeMap<String, usize>) -> CodeModel {
        let mut relationships = BTreeMap::new();
        for (word, _) in word_counts {
            // Placeholder: Build relationships (e.g., co-occurrence analysis)
            relationships.insert(word.clone(), vec![]);
        }
        CodeModel {
            term_frequencies: word_counts.clone(),
            relationships,
        }
    }
}

/// Module for function generation
mod function_generator {
    use std::collections::BTreeMap;

    pub fn generate_function(name: &str, stats: &BTreeMap<String, usize>) -> String {
        // Placeholder: Generate a Rust function based on statistics
        format!("fn {}() {{ /* Generated based on stats: {:?} */ }}", name, stats)
    }
}

/// Module for dataset export
mod dataset_exporter {
    use std::fs;
    use std::path::Path;
    use serde_json::{Value, json};

    pub fn create_hf_dataset(analyses: &[super::ast_parser::Analysis], dataset_dir: &str) {
        if !Path::new(dataset_dir).exists() {
            fs::create_dir_all(dataset_dir).unwrap();
        }

        let data_dir = format!("{}/data", dataset_dir);
        fs::create_dir_all(&data_dir).unwrap();

        let mut current_chunk = Vec::new();
        let mut current_chunk_size = 0;
        let mut chunk_index = 0;
        let max_file_size = 1024 * 1024; // 1MB

        for analysis in analyses {
            if let Ok(ast) = serde_json::from_str::<Value>(&analysis.json_ast) {
                let mut type_counts = BTreeMap::new();
                let mut total_nodes = 0;
                super::analyzer::count_types_recursive(&ast, &mut type_counts, &mut total_nodes);
                let example = json!({
                    "file_path": analysis.file_path,
                    "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                    "ast": ast,
                    "summary": {
                        "total_nodes": total_nodes,
                        "type_counts": type_counts,
                    }
                });

                let example_json = serde_json::to_string(&example).unwrap();
                let example_size = example_json.len();

                if current_chunk_size + example_size > max_file_size && !current_chunk.is_empty() {
                    let chunk_path = format!("{}/chunk_{:05}.json", data_dir, chunk_index);
                    let chunk_data = json!({
                        "examples": current_chunk,
                        "metadata": {
                            "chunk_index": chunk_index,
                            "num_examples": current_chunk.len(),
                            "total_size_bytes": current_chunk_size
                        }
                    });
                    fs::write(&chunk_path, serde_json::to_string(&chunk_data).unwrap()).unwrap();
                    current_chunk.clear();
                    current_chunk_size = 0;
                    chunk_index += 1;
                }

                current_chunk.push(example);
                current_chunk_size += example_size;
            }
        }

        if !current_chunk.is_empty() {
            let chunk_path = format!("{}/chunk_{:05}.json", data_dir, chunk_index);
            let chunk_data = json!({
                "examples": current_chunk,
                "metadata": {
                    "chunk_index": chunk_index,
                    "num_examples": current_chunk.len(),
                    "total_size_bytes": current_chunk_size
                }
            });
            fs::write(&chunk_path, serde_json::to_string(&chunk_data).unwrap()).unwrap();
        }

        let dataset_info = json!({
            "description": "Rust codebase AST analysis",
            "license": "agpl-3.0",
            "version": {"version_str": "0.1.0"}
        });
        fs::write(format!("{}/dataset_info.json", dataset_dir), serde_json::to_string_pretty(&dataset_info).unwrap()).unwrap();
    }
}

/// Module for LLM feedback loop
mod llm_feedback {
    use std::collections::BTreeMap;

    pub fn generate_new_functions(stats: &BTreeMap<String, usize>) -> Vec<String> {
        // Placeholder: Simulate LLM generating new functions
        let mut functions = Vec::new();
        for (ty, count) in stats {
            if *count > 5 { // Arbitrary threshold
                functions.push(super::function_generator::generate_function(&format!("handle_{}", ty), stats));
            }
        }
        functions
    }
}

const EMOJI_TYPE_MAP: &[(&str, &str, &str)] = &[
    ("fn", "🦀⚙️", "Rust Core"),
    ("struct", "🏛️🧱", "Rust Core"),
    // ... (rest of the emoji map as provided)
];

fn main() {
    // 1. Filter Rust files
    let mut files = HashMap::new();
    for entry in WalkDir::new("src").into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |e| e == "rs") {
            let path = entry.path().to_string_lossy().to_string();
            if let Ok(content) = fs::read_to_string(entry.path()) {
                files.insert(path, content);
            }
        }
    }

    if files.is_empty() {
        println!("[WARN] No Rust files found. Exiting.");
        return;
    }

    // 2. Parse to AST
    let mut analyses = Vec::new();
    for (path, content) in files {
        if let Ok(ast) = ast_parser::parse_rust_file(&content) {
            let json_ast = ast_parser::ast_to_json(&ast);
            analyses.push(ast_parser::Analysis { file_path: path, json_ast });
        }
    }

    // 3. Analyze and embed
    let mut type_counts = BTreeMap::new();
    let mut total_nodes = 0;
    let mut string_literals = Vec::new();
    for analysis in &analyses {
        if let Ok(ast) = serde_json::from_str::<Value>(&analysis.json_ast) {
            analyzer::count_types_recursive(&ast, &mut type_counts, &mut total_nodes);
            emoji_mapper::extract_string_literals(&ast, &mut string_literals);
        }
    }

    // 4. Detect duplicates
    let duplicates = duplicate_detector::find_duplicates(&analyses);
    for (ast, paths) in duplicates {
        println!("[DUPLICATE] AST found in: {:?}", paths);
    }

    // 5. Analyze statistics
    let mut word_counts = BTreeMap::new();
    for s in string_literals {
        for word in emoji_mapper::split_words(&s) {
            *word_counts.entry(word).or_insert(0) += 1;
        }
    }

    // 6. Build mathematical model
    let model = mathematical_model::build_model(&word_counts);

    // 7. Generate new functions
    let new_functions = llm_feedback::generate_new_functions(&type_counts);
    for func in new_functions {
        println!("[NEW FUNCTION] {}", func);
    }

    // 8. Export dataset
    dataset_exporter::create_hf_dataset(&analyses, "hf_dataset");

    // 9. Recursive LLM feedback loop
    // Placeholder: Continuously analyze statistics and generate new functions
    let mut iteration = 0;
    while iteration < 3 { // Limited iterations for demonstration
        let new_stats = type_counts.clone(); // Simulate new analysis
        let more_functions = llm_feedback::generate_new_functions(&new_stats);
        for func in more_functions {
            println!("[LLM ITERATION {}] {}", iteration, func);
        }
        iteration += 1;
    }
}
```

#### 3. **Addressing Your Idea**
Here's how the refactored code addresses each part of your idea:

- **Filter Rust Files**: The `main` function uses `WalkDir` to collect `.rs` files from the `src` directory.
- **Parse to AST**: The `ast_parser` module uses the `syn` crate to parse Rust code into an AST and convert it to JSON.
- **Embed Source into Vector**: The `analyzer` module counts AST node types and extracts string literals, which can be used as features for vector embedding (e.g., using an external embedding model like BERT).
- **Report and Identify Duplicates**: The `duplicate_detector` module identifies files with identical ASTs.
- **Analysis of Statistics**: The `analyzer` module collects type counts and word frequencies, providing statistical insights.
- **Construction of New Functions**: The `function_generator` module generates placeholder Rust functions based on statistics, which can be extended with LLM-generated code.
- **Extraction of Terms and Relationships**: The `mathematical_model` module extracts term frequencies and placeholders for relationship analysis (e.g., co-occurrence).
- **Mathematical Model**: The `mathematical_model` module builds a basic model, which can be extended with graph-based or probabilistic models.
- **Export Data for GUI**: The `dataset_exporter` module creates a Hugging Face dataset with compact JSON files, suitable for GUI visualization.
- **Recursive Feedback Loop**: The `llm_feedback` module simulates an LLM generating new functions based on statistics, with a loop for iterative refinement.

#### 4. **Recursive LLM Feedback Loop**
To implement a real-time feedback loop using LLMs:
- **Integration**: Use an external LLM API (e.g., via `reqwest` to call an endpoint like xAI's API) to generate function code based on statistics.
- **Process**:
  1. Collect statistics (e.g., `type_counts`, `word_counts`).
  2. Send statistics to the LLM with a prompt like: "Generate Rust functions to handle common AST node types: {stats}."
  3. Parse the LLM's response to extract valid Rust code.
  4. Re-analyze the codebase with the new functions included.
  5. Repeat until convergence or a fixed number of iterations.
- **Example Prompt**:
  ```text
  Given the following AST node statistics: {type_counts}, generate Rust functions to process the most frequent node types. Ensure the functions are safe and idiomatic.
  ```
- **Implementation Note**: The current code uses a placeholder in `llm_feedback`. To integrate a real LLM, add an HTTP client and API key handling.

#### 5. **Mathematical Model**
The `mathematical_model` module is a placeholder. To enhance it:
- **Term Relationships**: Use co-occurrence analysis or dependency graphs to model relationships between terms.
- **Vector Embeddings**: Convert string literals and AST nodes to vectors using a pre-trained model (e.g., via `rust-bert`).
- **Graph Model**: Build a graph where nodes are terms and edges represent co-occurrence or syntactic relationships.

#### 6. **Export for GUI**
The dataset is exported in a Hugging Face-compatible format with JSON chunks and metadata. To reduce size:
- **Compression**: Use `flate2` to compress JSON files.
- **Filtering**: Exclude low-frequency terms or nodes with counts below a threshold.
- **GUI Integration**: Serve the dataset via a web server (e.g., using `actix-web`) or visualize with a frontend framework like Dioxus.

#### 7. **Challenges and Notes**
- **Missing CodeAnalyzer**: The original code relies on `solfunmeme_dioxus::core::code_analyzer::CodeAnalyzer`. Without its implementation, I assumed a basic AST parsing functionality. Please provide details if it does more (e.g., vector embeddings).
- **AST to JSON**: The `ast_parser::ast_to_json` function is a placeholder. Use a library like `serde_syn` or manually serialize `syn::File` to JSON.
- **LLM Integration**: Real LLM integration requires an API client and careful prompt engineering.
- **Performance**: For large codebases, consider parallel processing with `rayon` or chunking the analysis.

#### 8. **Next Steps**
- **Provide CodeAnalyzer Details**: Share the implementation or specify its functionality.
- **LLM API**: Choose an LLM provider (e.g., xAI's API at https://x.ai/api) and integrate it.
- **Visualization**: Build a GUI using Dioxus or a web framework to display the dataset and generated functions.
- **Model Enhancement**: Implement a robust mathematical model (e.g., graph-based or probabilistic).

If you have specific questions or need help with any part (e.g., LLM integration, GUI setup), let me know!