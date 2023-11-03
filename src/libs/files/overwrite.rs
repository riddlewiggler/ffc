use anyhow::{anyhow, Result};
use log::{error, trace};
use std::fs::rename;
use std::path::Path;

pub fn overwrite_file(source: &Path, temp: &Path) -> Result<bool> {
    trace!("Overwriting source file {}", source.display());
    rename(temp, source).map_err(|e| {
        error!("{e}");
        let msg = format!("Failed to overwrite {}", source.display());
        anyhow!(msg)
    })?;
    Ok(true)
}

#[cfg(test)]
#[path = "./__tests__/overwrite.spec.rs"]
mod overwrite_test;
