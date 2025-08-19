use zip::write::{FileOptions, ZipWriter};
use zip::read::ZipArchive;
use std::io::{Write, Read};
use std::fs::File;
use std::path::Path;
use anyhow::{Result, anyhow};

pub fn create_zip_archive(files: &[(&Path, &[u8])], output_path: &Path) -> Result<()> {
    let file = File::create(output_path)?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    for (path, content) in files {
        zip.start_file(path.to_str().ok_or_else(|| anyhow!("Invalid path"))?, options)?;
        zip.write_all(content)?;
    }
    zip.finish()?;
    Ok(())
}

pub fn extract_zip_archive(archive_path: &Path, extract_to: &Path) -> Result<()> {
    let file = File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => extract_to.join(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}
