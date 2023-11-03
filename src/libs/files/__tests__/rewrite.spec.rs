#[cfg(test)]
use super::rewrite_file;
use anyhow::Result;
use std::fs::remove_file;
use std::{fs, path::Path};

static SOURCE_FILE_A: &str = "tests/mocks/pristine/rewrite.txt";
static FILE_A: &str = "tests/mocks/rewrite.txt";
static FILE_B: &str = "tests/mocks/rewrite.tmp.txt";

fn before_each() -> Result<()> {
    fs::copy(SOURCE_FILE_A, FILE_A)?;
    // fs::copy(SOURCE_FILE_B, FILE_B)?;
    Ok(())
}

fn after_each() -> Result<()> {
    delete_file(FILE_A)?;
    delete_file(FILE_B)?;
    Ok(())
}

#[test]
fn true_if_file_is_rewritten() -> Result<()> {
    before_each()?;

    let a = Path::new(FILE_A);
    let b = Path::new(FILE_B);
    let ret = rewrite_file(a, b);
    assert_eq!(ret.unwrap(), true);

    after_each()?;
    Ok(())
}

#[test]
fn error_if_file_does_not_exist() {
    let a = Path::new("./not-existant-file.txt");
    let b = Path::new("./not-existant-file.temp.txt");
    let ret = rewrite_file(a, b);
    assert_eq!(ret.is_err(), true);
}

fn delete_file(filepath: &str) -> Result<()> {
    if Path::new(filepath).exists() == true {
        remove_file(filepath)?;
    }

    Ok(())
}
