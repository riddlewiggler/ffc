use super::{is_to_be_sanitized, overwrite_file, rewrite_file, FILENAME, TEMP_FILENAME};
use anyhow::Result;
use std::path::PathBuf;

pub fn sanitize_files(paths: Vec<PathBuf>) -> Result<u32> {
    let mut count = 0;
    for path in paths.iter() {
        let source_file = path.join(FILENAME);
        let temp_file = path.join(TEMP_FILENAME);

        let ret = is_to_be_sanitized(source_file.as_path())?;

        if ret == true {
            rewrite_file(source_file.as_path(), temp_file.as_path())?;
            overwrite_file(source_file.as_path(), temp_file.as_path())?;
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
#[path = "./__tests__/sanitize_files.spec.rs"]
mod sanitize_files_test;
