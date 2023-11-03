#[cfg(test)]
use super::sanitize_files;
use anyhow::Result;
use std::{
    fs::{self, remove_file},
    path::Path,
};

static SOURCE_FILE: &str = "tests/mocks/pristine/pkcs11.txt";
static FILE: &str = "tests/mocks/pkcs11.txt";
static FOLDER_PATH: &str = "tests/mocks";

fn before() -> Result<()> {
    fs::copy(SOURCE_FILE, FILE)?;
    Ok(())
}

fn after() -> Result<()> {
    delete_file(FILE)?;
    Ok(())
}

#[test]
fn test_sanitize_files() -> Result<()> {
    before()?;
    let a = Path::new(FOLDER_PATH).to_path_buf();
    let file_list = vec![a];
    sanitize_files(file_list)?;

    after()?;
    Ok(())
}

fn delete_file(filepath: &str) -> Result<()> {
    if Path::new(filepath).exists() == true {
        remove_file(filepath)?;
    }

    Ok(())
}
