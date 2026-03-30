use std::path::PathBuf;
use std::env;

#[derive(Debug)]
pub struct DocProcessorArgs {
    pub target_path: PathBuf,
    pub output_dir: PathBuf,
    pub limit: Option<usize>,
    pub include_pattern: Option<String>,
}

impl DocProcessorArgs {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut args = env::args().skip(1);
        let mut target_path: Option<String> = None;
        let mut limit: Option<usize> = None;
        let mut output_dir: Option<String> = None;
        let mut include_pattern: Option<String> = None;
        
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
                "--include" | "-i" => {
                    include_pattern = args.next();
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
            target_path: PathBuf::from(target_path.unwrap_or_else(|| ".".to_string())),
            output_dir: PathBuf::from(output_dir.unwrap_or_else(|| "processed_docs".to_string())),
            limit,
            include_pattern,
        })
    }
}

fn print_help() {
    println!("Usage: doc_processor [OPTIONS] [PATH]");
    println!();
    println!("Options:");
    println!("  --limit N           Limit processing to N files");
    println!("  --output, -o DIR    Output directory for processed files");
    println!("  --include, -i PATTERN Include files matching glob pattern (e.g., '*.md')");
    println!("  --help, -h          Show this help message");
    println!();
    println!("Examples:");
    println!("  doc_processor .                          # Process current directory");
    println!("  doc_processor --limit 100 .              # Process first 100 files");
    println!("  doc_processor -o processed_docs .       # Save to output directory");
    println!("  doc_processor -i '*.md' .               # Process only markdown files");
}
