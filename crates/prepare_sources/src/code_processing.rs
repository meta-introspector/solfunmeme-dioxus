use anyhow::Result;
use solfunmeme_input_fs::read_code_chunks;
use solfunmeme_function_analysis::CodeChunk;
use solfunmeme_clifford::SerializableMultivector;
use tclifford::Multivector;
use solfunmeme_language_processing::{LanguageProcessor, rust_processor::RustProcessor, markdown_processor::MarkdownProcessor};

pub fn process_code_chunks(path: Option<String>, limit: Option<usize>) -> Result<Vec<CodeChunk>> {
    let files_content = read_code_chunks(path, limit)?;
    eprintln!("[INFO] Processing {} files:", files_content.len());

    let mut all_chunks: Vec<CodeChunk> = Vec::new();
    let mut processed_count = 0;

    for (file_path, content) in files_content {
        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        let mut chunks: Vec<CodeChunk> = Vec::new();

        match ext.as_str() {
            "rs" => {
                let processor = RustProcessor;
                chunks = processor.process_code(&content, &file_path.to_string_lossy())?;
            },
            "md" => {
                let processor = MarkdownProcessor;
                chunks = processor.process_code(&content, &file_path.to_string_lossy())?;
            },
            _ => {
                eprintln!("[WARN] Skipping unsupported file type: {}", file_path.display());
                continue;
            }
        }

        for mut chunk in chunks {
            // Simulate embedding and Clifford vector generation
            let mut coeffs = [0.0; 8];
            // In a real scenario, you'd use a proper embedding model here.
            // For simulation, we'll just use some dummy values or a hash of the content.
            // For now, let's just use a simple incrementing value for demonstration.
            for i in 0..8 {
                coeffs[i] = (processed_count as f32 + i as f32) / 100.0;
            }
            let clifford_vector = SerializableMultivector(Multivector::from_vector(coeffs.to_vec()).unwrap());
            chunk.clifford_vector = Some(clifford_vector);
            chunk.embedding = coeffs.to_vec(); // Store the simulated embedding

            all_chunks.push(chunk);

            processed_count += 1;
            if processed_count % 100 == 0 {
                eprintln!("[INFO] Processed {} chunks so far...", processed_count);
            }
        }
    }
    eprintln!("[INFO] Finished processing all {} chunks.", processed_count);

    Ok(all_chunks)
}