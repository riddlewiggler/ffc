#[cfg(test)]
use super::sanitize_browser_list;
use super::Browser::{Chrome, Firefox};
use anyhow::Result;
static HOME: &str = "tests/home";

#[test]
fn it_should_return_true_for_chrome() -> Result<()> {
    let home = Some(HOME.to_string());
    let browser_list = vec![Chrome.to_string(), "Unknown".to_string()];
    let ret = sanitize_browser_list(browser_list, home)?;
    assert_eq!(ret, true);

    Ok(())
}

#[test]
fn it_should_return_true_for_firefox() -> Result<()> {
    let home = Some(HOME.to_string());
    let browser_list = vec![Firefox.to_string(), "Unknown".to_string()];
    let ret = sanitize_browser_list(browser_list, home)?;
    assert_eq!(ret, true);

    Ok(())
}
