use super::FILENAME;
use crate::Browser;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn fmt_browser_filepaths(browser: &Browser, home_dir: &String) -> Result<Vec<PathBuf>> {
    let mut path = PathBuf::from(&home_dir);
    let mut path_list: Vec<PathBuf> = Vec::new();

    if *browser == Browser::Chrome {
        path.push(".pki");
        path.push("nssdb");

        let base_path = path.clone();

        path.push(FILENAME);

        if path.exists() == true {
            path_list.push(base_path);
        }
    } else if *browser == Browser::Firefox {
        path.push(".mozilla");
        path.push("firefox");
        let paths = std::fs::read_dir(&path).context("Reading Firefox folder failed")?;

        for p in paths {
            let base_path = p.context("Getting Firefox iterated path")?.path();
            if base_path.is_dir() == true {
                let mut file_path = base_path.clone();
                file_path.push(FILENAME);
                if file_path.exists() == true {
                    path_list.push(base_path);
                }
            }
        }
    }

    Ok(path_list)
}

#[cfg(test)]
#[path = "./__tests__/fmt_browser_filepaths.spec.rs"]
mod fmt_browser_filepaths_test;