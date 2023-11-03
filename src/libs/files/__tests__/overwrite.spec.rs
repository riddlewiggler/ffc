#[cfg(test)]
use super::overwrite_file;
use anyhow::Result;
use std::{
    fs::{remove_file, File},
    io::Write,
    path::Path,
};
static FILE: &str = "tests/mocks/overwrite.txt";
static TEMP_FILE: &str = "tests/mocks/overwrite.temp.txt";
static NOT_FILE: &str = "tests/mocks/not-existant.txt";

fn before() -> Result<()> {
    create_file(TEMP_FILE)?;
    Ok(())
}

fn after() -> Result<()> {
    delete_file(TEMP_FILE)?;
    delete_file(FILE)?;
    Ok(())
}

#[test]
fn it_should_overwrite_file() -> Result<()> {
    before()?;

    let a = Path::new(FILE);
    let b = Path::new(TEMP_FILE);
    let ret = overwrite_file(a, b)?;
    assert_eq!(ret, true);

    after()?;
    Ok(())
}

#[test]
fn it_should_fail_overwrite_file() -> Result<()> {
    let a = Path::new(NOT_FILE);
    let b = Path::new(TEMP_FILE);
    let ret = overwrite_file(a, b);
    assert_eq!(ret.is_err(), true);

    Ok(())
}

fn create_file(filepath: &str) -> Result<()> {
    let mut file = File::create(filepath)?;
    file.write_all(b"Hello, world!")?;
    file.flush()?;
    Ok(())
}

fn delete_file(filepath: &str) -> Result<()> {
    if Path::new(filepath).exists() == true {
        remove_file(filepath)?;
    }

    Ok(())
}
