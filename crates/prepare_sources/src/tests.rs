use super::*;
use std::fs;

#[test]
fn it_works() {
    // Create a dummy file for testing
    let file_path = "test.txt";
    fs::write(file_path, "This is a test.").unwrap();

    let result = process_file(file_path);
    assert!(result.is_ok());

    // Clean up the dummy file
    fs::remove_file(file_path).unwrap();
}
