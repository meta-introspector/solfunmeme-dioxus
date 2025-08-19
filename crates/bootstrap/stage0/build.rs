// crates/bootstrap/stage0/build.rs

use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=components.manifest");

    let manifest_path = Path::new("components.manifest");
    if !manifest_path.exists() {
        panic!("components.manifest not found. Please create it.");
    }

    let content = fs::read_to_string(manifest_path)
        .expect("Failed to read components.manifest");

    let mut names = HashSet::new();
    for (line_num, line) in content.lines().enumerate() {
        let name = line.trim();
        if name.is_empty() {
            continue;
        }
        if !names.insert(name) {
            panic!(
                "Duplicate component name found in components.manifest on line {}: '{}'",
                line_num + 1,
                name
            );
        }
    }
}
