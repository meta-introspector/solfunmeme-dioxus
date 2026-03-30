use std::path::PathBuf;
use std::env;

#[derive(Debug)]
pub struct DocTestGeneratorArgs {
    pub input_file: PathBuf,
    pub output_dir: PathBuf,
}

impl DocTestGeneratorArgs {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut args = env::args().skip(1);
        let mut input_file: Option<String> = None;
        let mut output_dir: Option<String> = None;
        
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--output" | "-o" => {
                    output_dir = args.next();
                },
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                },
                _ => {
                    if input_file.is_none() {
                        input_file = Some(arg);
                    }
                }
            }
        }

        Ok(Self {
            input_file: PathBuf::from(input_file.unwrap_or_else(|| "README.md".to_string())),
            output_dir: PathBuf::from(output_dir.unwrap_or_else(|| "tests".to_string())),
        })
    }
}

fn print_help() {
    println!("Usage: doc_test_generator [OPTIONS] [INPUT_FILE]");
    println!();
    println!("Options:");
    println!("  --output, -o DIR    Output directory for generated tests (default: tests)");
    println!("  --help, -h          Show this help message");
    println!();
    println!("Examples:");
    println!("  doc_test_generator README.md              # Generate tests from README.md");
    println!("  doc_test_generator -o my_tests README.md  # Save tests to my_tests directory");
}
