use solfunmeme_tools::chat_processing::ChunkProcessor;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_process_document() {
    let output_dir = tempdir().unwrap();
    let processor = ChunkProcessor::new(output_dir.path().to_path_buf());

    let fixture_path = Path::new("tests/fixtures/chunk_processor_test.md");
    let fixture_content = fs::read_to_string(fixture_path).unwrap();

    processor
        .process_document(fixture_path, &fixture_content)
        .unwrap();

    // Verify the output directory structure
    // YYYY/MM/DD is based on file modification time, so it's tricky to test precisely without mocking time.
    // For now, we'll just check for the presence of the normalized filename directory.
    let normalized_name = "chunk_processor_test";
    let doc_dir_glob = output_dir
        .path()
        .join("*/*/*")
        .join(normalized_name)
        .to_str()
        .unwrap()
        .to_string();

    let mut doc_dirs: Vec<_> = glob::glob(&doc_dir_glob).unwrap().filter_map(Result::ok).collect();
    assert_eq!(doc_dirs.len(), 1);
    let doc_dir = doc_dirs.pop().unwrap();

    assert!(doc_dir.join("index.md").exists());
    assert!(doc_dir.join("chunk_0/content.md").exists());
    assert!(doc_dir.join("chunk_0/metadata.json").exists());
    assert!(doc_dir.join("chunk_1/content.md").exists());
    assert!(doc_dir.join("chunk_1/metadata.json").exists());
}
