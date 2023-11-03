#[cfg(test)]
use super::fmt_browser_filepaths;
use crate::Browser::{Chrome, Firefox};
use anyhow::Result;
static HOME: &str = "tests/home";

#[test]
fn it_should_fmt_chrome_paths() -> Result<()> {
    let ret = fmt_browser_filepaths(&Chrome, &HOME.to_string())?;
    assert_eq!(ret.len(), 0);

    Ok(())
}

#[test]
fn it_should_fmt_firefox_paths() -> Result<()> {
    let ret = fmt_browser_filepaths(&Firefox, &HOME.to_string())?;
    assert_eq!(ret.len(), 0);

    Ok(())
}
