use super::files::get_user_home_dir;
use super::sanitize_browser::sanitize_browser;
use super::Browser;
use anyhow::{anyhow, Result};
use log::error;

/// Sanitize the certificate files (pkcs11) of a list of browsers
/// 
/// # Arguments
/// 
/// * `browsers` - List of supported browsers, namely chrome and firefox
/// * `user_home` - Optional user base folder. Helpful injection during tests
pub fn sanitize_browser_list(browsers: Vec<String>, user_home: Option<String>) -> Result<bool> {
    let c = Browser::Chrome.to_string().to_lowercase();
    let f = Browser::Firefox.to_string().to_lowercase();
    let mut status: bool = true;
    let default_home = get_user_home_dir()?;
    let home = user_home.unwrap_or(default_home);

    for browser_arg in browsers.iter() {
        let b = browser_arg.to_lowercase();

        let browser = match b {
            x if x.contains(&c) => Browser::Chrome,
            x if x.contains(&f) => Browser::Firefox,
            _ => {
                error!("Unrecognized browser: {b}. Skipping.");
                continue;
            }
        };

        let home_arg = Some(home.clone());

        if let Err(e) = sanitize_browser(browser, home_arg) {
            error!("{e:?}");
            status = false;
        }
    }

    match status {
        true => Ok(true),
        false => Err(anyhow!("FFC sanitization failed")),
    }
}

#[cfg(test)]
#[path = "./__tests__/sanitize_browser_list.spec.rs"]
mod sanitize_browser_list_test;
