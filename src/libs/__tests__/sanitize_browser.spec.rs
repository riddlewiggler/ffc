#[cfg(test)]
use super::sanitize_browser;
use crate::Browser::{Chrome, Firefox};
use anyhow::Result;
static HOME: &str = "tests/home";

#[test]
fn it_should_return_true_for_chrome() -> Result<()> {
    let home = Some(HOME.to_string());
    let ret = sanitize_browser(Chrome, home)?;
    assert_eq!(ret, true);

    Ok(())
}

#[test]
fn it_should_return_true_for_firefox() -> Result<()> {
    let home = Some(HOME.to_string());
    let ret = sanitize_browser(Firefox, home)?;
    assert_eq!(ret, true);

    Ok(())
}