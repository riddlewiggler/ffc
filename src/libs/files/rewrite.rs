use anyhow::{Result, anyhow};
use log::{error, trace};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub fn rewrite_file(source_file: &Path, temp_file: &Path) -> Result<bool> {
    trace!("Opening file {}", source_file.display());
    let file = File::open(source_file).map_err(|e| {
        error!("{e}");
        let msg = format!("Failed to open {}", source_file.display());
        anyhow!(msg)
    })?;

    // Copying all lines from source to temp file but those containing forty client
    trace!("Creating temp file {}", temp_file.display());
    let out_file = File::create(temp_file).map_err(|e| {
        error!("{e}");
        let msg = format!("Failed to create {}", temp_file.display());
        anyhow!(msg)
    })?;

    let reader = BufReader::new(&file);
    let mut writer = BufWriter::new(&out_file);

    let line_fallback = String::from("");

    for (idx, line) in reader.lines().enumerate() {
        let line = line.as_ref().unwrap_or_else(|e| {
            error!("{e}");
            &line_fallback
        });

        if line.to_lowercase().contains("forticlient") == false {
            writeln!(writer, "{}", line).map_err(|e| {
                error!("{e}");
                let msg = format!("Failed to write line {idx} to {}", temp_file.display());
                anyhow!(msg)
            })?;
        }
    }

    writer.flush()?;

    Ok(true)
}

#[cfg(test)]
#[path = "./__tests__/rewrite.spec.rs"]
mod rewrite_test;