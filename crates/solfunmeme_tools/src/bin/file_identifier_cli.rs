use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[command(name = "file_identifier")]
#[command(about = "Identify file types and sizes in directories", long_about = None)]
struct Cli {
    #[arg(help = "Paths to directories to analyze")]
    directories: Vec<PathBuf>,

    #[arg(long, help = "Show only files larger than this size (in bytes)")]
    min_size: Option<u64>,

    #[arg(long, help = "Show only files smaller than this size (in bytes)")]
    max_size: Option<u64>,

    #[arg(long, help = "Limit number of files shown per extension")]
    limit: Option<usize>,

    #[arg(long, default_value_t = 0, help = "Verbosity level (0-9)")]
    verbose: u8,
}

fn get_magic_header(path: &PathBuf, n: usize) -> Option<String> {
    let mut file = File::open(path).ok()?;
    let mut buf = vec![0u8; n];
    let n_read = file.read(&mut buf).ok()?;
    Some(buf[..n_read].iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" "))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.directories.is_empty() {
        println!("Usage: file_identifier <directory1> [directory2 ...] [--min-size N --max-size N --limit N --verbose=9]");
        println!("Example: file_identifier vendor/ --min-size 1000000 --limit 10 --verbose=9");
        return Ok(());
    }

    let mut extension_stats: HashMap<String, Vec<(PathBuf, u64)>> = HashMap::new();
    let mut total_files = 0;
    let mut total_size = 0;

    if cli.verbose > 0 {
        println!("\n--- File Identification (verbose={}) ---", cli.verbose);
    }

    for dir_path in &cli.directories {
        if cli.verbose > 0 {
            println!("Analyzing directory: {}", dir_path.display());
        }
        for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let file_path = entry.path().to_path_buf();
                let ext = file_path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("no-ext")
                    .to_lowercase();
                let file_size = entry.metadata().map(|m| m.len()).unwrap_or(0);

                // Apply size filters
                if let Some(min) = cli.min_size {
                    if file_size < min {
                        continue;
                    }
                }
                if let Some(max) = cli.max_size {
                    if file_size > max {
                        continue;
                    }
                }

                total_files += 1;
                total_size += file_size;
                extension_stats.entry(ext.clone()).or_insert_with(Vec::new).push((file_path.clone(), file_size));

                if cli.verbose >= 9 {
                    let magic = get_magic_header(&file_path, 16).unwrap_or_else(|| "(unreadable)".to_string());
                    println!("{} | {} bytes | .{} | magic: {}", file_path.display(), file_size, ext, magic);
                }
            }
        }
    }

    if cli.verbose < 9 {
        println!("\n--- Summary ---");
        println!("Total files: {}", total_files);
        println!("Total size: {} bytes ({:.2} MB)", total_size, total_size as f64 / 1024.0 / 1024.0);

        println!("\n--- File Types by Size ---");
        let mut sorted_extensions: Vec<(String, Vec<(PathBuf, u64)>)> = extension_stats.into_iter().collect();
        sorted_extensions.sort_by(|a, b| {
            let a_total: u64 = a.1.iter().map(|(_, size)| size).sum();
            let b_total: u64 = b.1.iter().map(|(_, size)| size).sum();
            b_total.cmp(&a_total) // Sort by total size, descending
        });

        for (ext, files) in sorted_extensions {
            let total_size: u64 = files.iter().map(|(_, size)| size).sum();
            let avg_size = total_size as f64 / files.len() as f64;
            let max_size = files.iter().map(|(_, size)| size).max().unwrap_or(0);
            
            println!("\n📁 {} ({} files, {:.2} MB total, {:.2} KB avg, {:.2} MB max):", 
                ext, files.len(), total_size as f64 / 1024.0 / 1024.0, 
                avg_size / 1024.0, max_size as f64 / 1024.0 / 1024.0);

            // Sort files by size (descending) and show top ones
            let mut sorted_files = files;
            sorted_files.sort_by(|a, b| b.1.cmp(&a.1));
            
            let limit = cli.limit.unwrap_or(5);
            for (i, (file_path, size)) in sorted_files.iter().take(limit).enumerate() {
                let size_mb = *size as f64 / 1024.0 / 1024.0;
                println!("  {}. {} ({:.2} MB)", i + 1, file_path.display(), size_mb);
            }
            
            if sorted_files.len() > limit {
                println!("  ... and {} more files", sorted_files.len() - limit);
            }
        }

        // Suggest skip extensions for large files
        println!("\n--- Suggested Skip Extensions ---");
        let large_extensions: Vec<_> = sorted_extensions.iter()
            .filter(|(_, files)| {
                let total_size: u64 = files.iter().map(|(_, size)| size).sum();
                total_size > 10 * 1024 * 1024 // More than 10MB total
            })
            .map(|(ext, _)| ext.clone())
            .collect();

        if !large_extensions.is_empty() {
            println!("Large file types to consider skipping:");
            println!("--skip-ext {}", large_extensions.join(","));
        }
    }

    Ok(())
} 