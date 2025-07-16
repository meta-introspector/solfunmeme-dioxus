use clap::{App, Arg, SubCommand};
use doc_cross_references::{VibeFinder, DuplicateFinder};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let matches = App::new("doc-cross-references")
        .version("1.0")
        .about("Find code patterns and vibes in your codebase")
        .subcommand(
            SubCommand::with_name("find-duplicates")
                .about("Find duplicate code patterns")
                .arg(
                    Arg::with_name("path")
                        .help("Path to search for Rust files")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("similarity")
                        .help("Similarity threshold (0.0-1.0)")
                        .short("s")
                        .long("similarity")
                        .default_value("0.8"),
                ),
        )
        .subcommand(
            SubCommand::with_name("find-similar")
                .about("Find similar code patterns")
                .arg(
                    Arg::with_name("path")
                        .help("Path to search for Rust files")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("query")
                        .help("Code pattern to search for")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("limit")
                        .help("Maximum number of results")
                        .short("l")
                        .long("limit")
                        .default_value("10"),
                ),
        )
        .subcommand(
            SubCommand::with_name("index-vibes")
                .about("Index code for vibe searching")
                .arg(
                    Arg::with_name("path")
                        .help("Path to index")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("index-dir")
                        .help("Directory to store Tantivy index")
                        .short("i")
                        .long("index-dir")
                        .default_value("./vibe_index"),
                ),
        )
        .subcommand(
            SubCommand::with_name("find-vibes")
                .about("Find code that matches a chat vibe")
                .arg(
                    Arg::with_name("chat-text")
                        .help("Chat text to match against")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("index-dir")
                        .help("Directory containing Tantivy index")
                        .short("i")
                        .long("index-dir")
                        .default_value("./vibe_index"),
                )
                .arg(
                    Arg::with_name("limit")
                        .help("Maximum number of results")
                        .short("l")
                        .long("limit")
                        .default_value("10"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("find-duplicates", Some(args)) => {
            let path = args.value_of("path").unwrap();
            let similarity: f32 = args.value_of("similarity").unwrap().parse()?;
            
            println!("Finding duplicates in: {}", path);
            let finder = DuplicateFinder::new();
            let duplicates = finder.find_duplicates(path, similarity)?;
            
            println!("Found {} duplicate groups:", duplicates.len());
            for (i, group) in duplicates.iter().enumerate() {
                println!("\nGroup {} ({} files):", i + 1, group.len());
                for duplicate in group {
                    println!("  - {}:{}-{}", duplicate.file_path, duplicate.line_start, duplicate.line_end);
                }
            }
        }
        
        ("find-similar", Some(args)) => {
            let path = args.value_of("path").unwrap();
            let query = args.value_of("query").unwrap();
            let limit: usize = args.value_of("limit").unwrap().parse()?;
            
            println!("Finding similar code to: {}", query);
            let finder = DuplicateFinder::new();
            let similar = finder.find_similar_code(path, query, limit)?;
            
            println!("Found {} similar code chunks:", similar.len());
            for (i, result) in similar.iter().enumerate() {
                println!("\n{}. Similarity: {:.3}", i + 1, result.similarity);
                println!("   File: {}:{}-{}", result.file_path, result.line_start, result.line_end);
                println!("   Code: {}", result.code.lines().next().unwrap_or(""));
            }
        }
        
        ("index-vibes", Some(args)) => {
            let path = args.value_of("path").unwrap();
            let index_dir = args.value_of("index-dir").unwrap();
            
            println!("Indexing code for vibe searching: {}", path);
            let mut finder = VibeFinder::new(Path::new(index_dir))?;
            finder.index_directory(Path::new(path))?;
            
            println!("Successfully indexed code for vibe searching!");
        }
        
        ("find-vibes", Some(args)) => {
            let chat_text = args.value_of("chat-text").unwrap();
            let index_dir = args.value_of("index-dir").unwrap();
            let limit: usize = args.value_of("limit").unwrap().parse()?;
            
            println!("Finding code that matches vibe: {}", chat_text);
            let finder = VibeFinder::new(Path::new(index_dir))?;
            let results = finder.find_vibe_matches(chat_text, limit)?;
            
            let report = finder.generate_vibe_report(chat_text, &results);
            println!("{}", report);
        }
        
        _ => {
            println!("{}", matches.usage());
        }
    }

    Ok(())
} 