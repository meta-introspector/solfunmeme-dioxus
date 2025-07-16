use anyhow::{Result};
use solfunmeme_tantivy_report::{get_top_entries, ReportType};
use std::path::Path;

pub fn report_top_entries(index_path: &Path, report_type: &str, limit: usize) -> Result<()> {
    let report_type = ReportType::Field(report_type.to_string());
    let top_entries = get_top_entries(index_path, report_type, limit)?;

    for (entry, count) in top_entries {
        println!("{}: {}", entry, count);
    }

    Ok(())
}
