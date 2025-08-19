use anyhow::{Result, anyhow};
use solfunmeme_function_analysis::CodeChunk;
use solfunmeme_search_tantivy::SearchIndex;
use solfunmeme_tantivy_report::{get_top_entries, ReportType};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::fs::File;
use tempfile::NamedTempFile;

pub fn index_directory(path: &Path, index_path: &Path, debug_backtrace: bool) -> Result<()> {
    let mut search_index = SearchIndex::new(index_path)?;

    // Create a temporary file to capture prepare_sources output
    let temp_output_file = NamedTempFile::new()?;
    let temp_output_path = temp_output_file.path().to_path_buf();

    eprintln!("[INFO] Running prepare_sources for: {}", path.display());
    let mut command = Command::new("cargo");
    command.args([
            "run",
            "-p",
            "prepare_sources",
            "--bin",
            "prepare_sources",
            "--",
            path.to_str().unwrap(),
            "--output",
            temp_output_path.to_str().unwrap(),
        ])
        .stdout(Stdio::null()) // Suppress stdout from prepare_sources
        .stderr(Stdio::inherit()); // Inherit stderr to see progress messages

    if debug_backtrace {
        command.env("RUST_BACKTRACE", "full");
    }

    let child = command.spawn()?;

    let output = child.wait_with_output()?;

    if !output.status.success() {
        return Err(anyhow!("prepare_sources failed with exit code: {:?}", output.status.code()));
    }

    eprintln!("[INFO] Reading chunks from temporary file: {}", temp_output_path.display());
    let file = File::open(&temp_output_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let chunk: CodeChunk = serde_json::from_str(&line)?;
        search_index.add_chunk(&chunk)?;
    }

    search_index.commit()
}

pub fn report_top_entries(index_path: &Path, report_type: &str, limit: usize) -> Result<()> {
    let report_type = ReportType::Field(report_type.to_string());
    let top_entries = get_top_entries(index_path, report_type, limit)?;

    for (entry, count) in top_entries {
        println!("{}: {}", entry, count);
    }

    Ok(())
}
