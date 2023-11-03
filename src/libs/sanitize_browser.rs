use super::files::{get_user_home_dir, sanitize_files, fmt_browser_filepaths};
use super::Browser;
use anyhow::Result;
use log::{info, trace};


/// Sanitize the certificate files (pkcs11) of the specified browser
/// 
/// # Arguments
/// 
/// * `browser` - Browser enum, namely Chrome and Firefox
/// * `user_home` - Optional user base folder. Helpful injection during tests
pub fn sanitize_browser(browser: Browser, user_home: Option<String>) -> Result<bool> {
    trace!("Sanitizing pkcs11.txt for {:?} browser", browser);
    let default_home = get_user_home_dir()?;
    let home = user_home.unwrap_or(default_home);

    let path_list = fmt_browser_filepaths(&browser, &home)?;

    let count = sanitize_files(path_list)?;
    info!("Sanitized {count} files for {:?} browser", browser);
    Ok(true)
}


#[cfg(test)]
#[path = "./__tests__/sanitize_browser.spec.rs"]
mod sanitize_browser_test;