use std::fs;
use std::path::Path;

fn main() {
    println!("Creating hierarchical vendor structure...");
    
    // Define the 7-5-3-2 structure
    let layers = vec!["crypto", "system", "network", "data", "compute", "ui", "ai"];
    
    let subcategories = vec![
        ("crypto", vec!["primitives", "protocols", "keys", "hashes", "utils"]),
        ("system", vec!["os", "io", "memory", "process", "utils"]),
        ("network", vec!["http", "tls", "websocket", "protocols", "utils"]),
        ("data", vec!["serialization", "storage", "structures", "formats", "utils"]),
        ("compute", vec!["math", "algorithms", "parallel", "optimization", "utils"]),
        ("ui", vec!["framework", "components", "rendering", "interaction", "utils"]),
        ("ai", vec!["models", "nlp", "embedding", "search", "utils"]),
    ];
    
    let components = vec!["core", "bindings", "extensions"];
    let levels = vec!["stable", "experimental"];
    
    let mut total_created = 0;
    
    for (layer, subcats) in subcategories {
        println!("Creating layer: {}", layer);
        
        for subcategory in subcats {
            for component in &components {
                for level in &levels {
                    let path = format!("{}/{}/{}/{}", layer, subcategory, component, level);
                    
                    if !Path::new(&path).exists() {
                        match fs::create_dir_all(&path) {
                            Ok(_) => {
                                println!("  Created: {}", path);
                                total_created += 1;
                            }
                            Err(e) => {
                                eprintln!("  Error creating {}: {}", path, e);
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("Structure creation complete!");
    println!("Total directories created: {}", total_created);
    println!("Structure: 7 layers × 5 subcategories × 3 components × 2 levels = 210 possible locations");
} 