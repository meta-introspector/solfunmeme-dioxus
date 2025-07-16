# `zip_plugin`

This crate provides functionalities for creating and extracting Zip archives within the Solfunmeme application.

## Purpose

It enables the project to efficiently bundle and unbundle collections of files, which can be useful for packaging code, distributing data, or managing project assets.

## Core Functionalities

-   **Create Zip Archive**: Compresses a set of files into a single Zip archive.
-   **Extract Zip Archive**: Extracts the contents of a Zip archive to a specified directory.

## Usage (Conceptual)

```rust
use zip_plugin::{create_zip_archive, extract_zip_archive};
use std::path::Path;
use std::fs;

fn main() {
    let output_zip = Path::new("my_archive.zip");
    let file_content = b"Hello, Zip!".to_vec();
    let files_to_archive = vec![(Path::new("file1.txt"), file_content.as_slice())];

    // Example: Create a Zip archive
    // create_zip_archive(&files_to_archive, output_zip).expect("Failed to create zip");
    // println!("Zip archive created: {}", output_zip.display());

    // Example: Extract a Zip archive
    // let extract_dir = Path::new("extracted_files");
    // fs::create_dir_all(extract_dir).expect("Failed to create extract directory");
    // extract_zip_archive(output_zip, extract_dir).expect("Failed to extract zip");
    // println!("Zip archive extracted to: {}", extract_dir.display());
}
```
