use std::path::Path;
use std::collections::{HashMap, HashSet};

pub fn path_to_camel_case(path: &std::path::Path) -> String {
    path.iter()
        .map(|os_str| {
            let s = os_str.to_string_lossy();
            s.split(|c: char| c == '-' || c == '_' || c == '.')
                .filter(|part| !part.is_empty())
                .map(|part| {
                    let mut chars = part.chars();
                    match chars.next() {
                        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                        None => String::new(),
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("")
}

// Sanitize struct name and append counter if name is already used
pub fn sanitize_struct_name(s: &str, used_names: &mut HashSet<String>) -> String {
    let base_name: String = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();

    let mut final_name = base_name.clone();
    let mut counter = 0;

    // Keep appending counter until the name is unique
    while !used_names.insert(final_name.clone()) {
        counter += 1;
        final_name = format!("{}{}", base_name, counter);
    }

    final_name
}

// Convert path to a valid module name, appending counter if necessary
pub fn path_to_module_name(path: &Path, module_name_counts: &mut HashMap<String, u32>) -> String {
    let base_name: String = path
        .iter()
        .map(|os_str| os_str.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("_")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();

    // Increment counter for this base_name
    let counter = module_name_counts.entry(base_name.clone()).or_insert(0);
    *counter += 1;

    // If counter is 1, use base_name; otherwise, append counter-1
    if *counter == 1 {
        base_name
    } else {
        format!("{}{}", base_name, *counter - 1)
    }
}
