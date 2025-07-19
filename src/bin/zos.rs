
use std::path::PathBuf;
use std::process;

use chat_indexer::ChatIndexer;
use vibe_prover::VibeProver;

use solfunmeme_core_utils::clap::Parser;
use zos_cli_modules::cli::Cli;
use zos_cli_modules::commands::Commands;
use zos_cli_modules::task_commands::TaskCommands;
use zos_cli_modules::chat_commands::ChatCommands;
use zos_cli_modules::config::Config;

fn main() {
    let cli = Cli::parse();
    let config = Config::load().expect("Failed to load configuration");

    match &cli.command {
        Commands::Vendor { path } => {
            println!("{}", format!("{}", config.vendor.message.replace("{}", &path.display().to_string())));
            // TODO: Implement vendor functionality
        }
        
        Commands::Index { source, output } => {
            println!("{}", format!("{}", config.index.message.replace("{}", &source.display().to_string()).replace("{}", &output.display().to_string())));
            // TODO: Implement indexing functionality
        }
        
        Commands::Dedup { source, threshold } => {
            println!("{}", format!("{}", config.dedup.message.replace("{}", &source.display().to_string()).replace("{}", &threshold.to_string())));
            // TODO: Implement deduplication functionality
        }
        
        Commands::Search { query, limit } => {
            println!("{}", format!("{}", config.search.message.replace("{}", &query).replace("{}", &limit.to_string())));
            // TODO: Implement search functionality
        }
        
        Commands::Tasks { operation } => {
            match operation {
                TaskCommands::List => {
                    println!("{}", config.tasks.list_message);
                    // TODO: Implement task listing
                }
                TaskCommands::Run { task_name } => {
                    println!("{}", format!("{}", config.tasks.run_message.replace("{}", &task_name)));
                    // TODO: Implement task execution
                }
                TaskCommands::Report { format } => {
                    println!("{}", format!("{}", config.tasks.report_message.replace("{}", &format)));
                    // TODO: Implement report generation
                }
            }
        }
        
        Commands::Analyze { format, output } => {
            println!("{}", format!("{}", config.analyze.message.replace("{}", &format).replace("{}", &output.display().to_string())));
            // TODO: Implement analysis functionality
        }
        
        Commands::Chat { operation } => {
            match operation {
                ChatCommands::Index { input, output } => {
                    println!("{}", format!("{}", config.chat.index_message.replace("{}", &input.display().to_string()).replace("{}", &output.display().to_string())));
                    let mut indexer = ChatIndexer::new(input.clone(), output.clone());
                    if let Err(e) = indexer.index_all() {
                        eprintln!("{}", format!("{} {}", config.chat.index_error, e));
                        process::exit(1);
                    }
                    println!("{}", config.chat.index_success);
                }
                
                
                
                ChatCommands::Vibe { ontology, output } => {
                    println!("{}", format!("{} {} {}", config.chat.vibe_message, ontology, output.display()));
                    let mut prover = VibeProver::new(ontology.clone());
                    if let Err(e) = prover.generate_proof(&output) {
                        eprintln!("{}", format!("{} {}", config.chat.vibe_error, e));
                        process::exit(1);
                    }
                    println!("{}", config.chat.vibe_success);
                }
                
                ChatCommands::Search { query, limit } => {
                    println!("{}", format!("{} {} {}", config.chat.search_message, query, limit));
                    let indexer = ChatIndexer::new(
                        PathBuf::from(&config.chat.default_processed_chats),
                        PathBuf::from(&config.chat.default_chat_index)
                    );
                    if let Ok(results) = indexer.search(&query, *limit) {
                        for result in results {
                            println!("{}", format!("{} {}", config.chat.search_found, result));
                        }
                    } else {
                        eprintln!("{}", config.chat.search_error);
                        process::exit(1);
                    }
                }
                
                ChatCommands::Analyze { analysis_type, format } => {
                    println!("{}", format!("{} {} {}", config.chat.analyze_message, analysis_type, format));
                    let indexer = ChatIndexer::new(
                        PathBuf::from(&config.chat.default_processed_chats),
                        PathBuf::from(&config.chat.default_chat_index)
                    );
                    if let Ok(analysis) = indexer.analyze(&analysis_type, &format) {
                        println!("{}", format!("{} {}", config.chat.analyze_result, analysis));
                    } else {
                        eprintln!("{}", config.chat.analyze_error);
                        process::exit(1);
                    }
                }
            }
        }
    }
}