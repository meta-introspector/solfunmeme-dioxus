use std::path::PathBuf;
use std::env;
use crate::utils;

#[derive(Debug)]
pub struct ChatProcessorArgs {
    pub target_path: PathBuf,
    pub output_dir: PathBuf,
    pub limit: Option<usize>,
}

impl ChatProcessorArgs {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut args = env::args().skip(1);
        let mut target_path: Option<String> = None;
        let mut limit: Option<usize> = None;
        let mut output_dir: Option<String> = None;
        
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--limit" => {
                    if let Some(lim) = args.next() {
                        limit = lim.parse().ok();
                    }
                },
                "--output" | "-o" => {
                    output_dir = args.next();
                },
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                },
                _ => {
                    if target_path.is_none() {
                        target_path = Some(arg);
                    }
                }
            }
        }

        Ok(Self {
            target_path: utils::normalize_path(&PathBuf::from(target_path.unwrap_or_else(|| ".".to_string()))),
            output_dir: utils::normalize_path(&PathBuf::from(output_dir.unwrap_or_else(|| "processed_chats".to_string()))),
            limit,
        })
    }
}

fn print_help() {
    println!("Usage: chat_processor [OPTIONS] [PATH]");
    println!();
    println!("Options:");
    println!("  --limit N           Limit processing to N files");
    println!("  --output, -o DIR    Output directory for processed files");
    println!("  --help, -h          Show this help message");
    println!();
    println!("Examples:");
    println!("  chat_processor .                          # Process current directory");
    println!("  chat_processor --limit 100 .              # Process first 100 files");
    println!("  chat_processor -o processed_chats .       # Save to output directory");
}
