use anyhow::{Result, anyhow};
use log::{error, trace};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn is_to_be_sanitized(filepath: &Path) -> Result<bool> where {
    trace!("Opening file {}", filepath.display());
    let file = File::open(filepath).map_err(|e| {
        error!("{e}");
        let msg = format!("Failed to open {}", filepath.display());
        anyhow!(msg)
    })?;

    let reader = BufReader::new(&file);
    let matched = is_there_a_match(reader)?;

    if matched == false {
        trace!("Found no forty client reference");
    }

    Ok(matched)
}

fn is_there_a_match(reader: BufReader<&File>) -> Result<bool> {
    let mut matched = false;

    for line in reader.lines() {
        if line?.to_lowercase().contains("forticlient") == true {
            matched = true;
            break;
        }
    }

    Ok(matched)
}

#[cfg(test)]
#[path = "./__tests__/is_to_be_sanitized.spec.rs"]
mod is_to_be_sanitized_test;