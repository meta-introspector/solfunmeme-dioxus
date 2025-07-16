use serde_json;
use serde::{Deserialize, Serialize};
use solfunmeme_input_fs::read_code_chunks;
use solfunmeme_function_analysis::CodeChunk;
use solfunmeme_embedding::embed_text;
use clap::Parser;
use std::io::{self, Write};
use std::fs::File;
use std::path::PathBuf;
use candle_core::Device;

#[derive(Parser)]
#[command(name = "prepare_sources")]
#[command(about = "Prepares source code by reading and chunking it into JSON format.", long_about = None)]
struct Cli {
    #[arg(help = "Path to the directory or file to process. Defaults to current directory.")]
    path: Option<PathBuf>,
    #[arg(long, help = "Limit processing to N files/chunks.")]
    limit: Option<usize>,
    #[arg(long, help = "Output file path for JSON chunks. If not specified, outputs to stdout.")]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let target_path = cli.path.map(|p| p.to_string_lossy().into_owned());
    let limit = cli.limit;

    let mut code_chunks = read_code_chunks(target_path, limit)?;

    eprintln!("[INFO] Processing {} chunks:", code_chunks.len());

    let device = Device::Cpu; // Use CPU for simplicity

    let mut output_writer: Box<dyn Write> = if let Some(output_path) = cli.output {
        Box::new(File::create(&output_path)?)
    } else {
        Box::new(io::stdout())
    };

    let mut processed_count = 0;
    for mut chunk in code_chunks.drain(..) {
        // Generate embedding for the chunk content
        match embed_text(&chunk.content, &device) {
            Ok(embedding) => {
                chunk.embedding = embedding;
            }
            Err(e) => {
                eprintln!("[WARN] Failed to generate embedding for chunk ({}...): {}", &chunk.content[..std::cmp::min(chunk.content.len(), 50)], e);
                // Optionally, you could skip this chunk or assign a default empty embedding
            }
        }

        let json_chunk = serde_json::to_string(&chunk)?;
        writeln!(output_writer, "{}", json_chunk)?;

        processed_count += 1;
        if processed_count % 100 == 0 {
            eprintln!("[INFO] Processed {} chunks so far...", processed_count);
        }
    }
    eprintln!("[INFO] Finished processing all {} chunks.", processed_count);

    Ok(())
}