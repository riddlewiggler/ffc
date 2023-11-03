#[cfg(test)]
use super::is_to_be_sanitized;
use anyhow::Result;
use std::{
    fs::{self, remove_file},
    path::Path,
};

static SOURCE_FILE_A: &str = "tests/mocks/pristine/reference.txt";
static FILE_A: &str = "tests/mocks/reference.txt";
static SOURCE_FILE_B: &str = "tests/mocks/pristine/no-reference.txt";
static FILE_B: &str = "tests/mocks/no-reference.txt";

fn before_each() -> Result<()> {
    fs::copy(SOURCE_FILE_A, FILE_A)?;
    fs::copy(SOURCE_FILE_B, FILE_B)?;
    Ok(())
}

fn after_each() -> Result<()> {
    delete_file(FILE_A)?;
    delete_file(FILE_B)?;
    Ok(())
}

#[test]
fn true_if_reference_is_found() -> Result<()> {
    before_each()?;

    let file = Path::new(FILE_A);
    let ret = is_to_be_sanitized(file);
    assert_eq!(ret.unwrap(), true);

    after_each()?;
    Ok(())
}

#[test]
fn false_if_reference_is_not_found() -> Result<()> {
    before_each()?;

    let file = Path::new(FILE_B);
    let ret = is_to_be_sanitized(file);
    assert_eq!(ret.unwrap(), false);

    after_each()?;
    Ok(())
}

#[test]
fn error_if_file_does_not_exist() {
    let file = Path::new("./not-existant-file.txt");
    let ret = is_to_be_sanitized(file);
    assert_eq!(ret.is_err(), true);
}

fn delete_file(filepath: &str) -> Result<()> {
    if Path::new(filepath).exists() == true {
        remove_file(filepath)?;
    }

    Ok(())
}
