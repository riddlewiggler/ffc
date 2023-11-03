use anyhow::{anyhow, Result};
use home::home_dir;
use log::trace;

pub fn get_user_home_dir() -> Result<String> {
    trace!("Retrieving user home folder");
    let home = match home_dir() {
        Some(path) => format!("{}", path.display()),
        None => {
            let msg = format!("Failed to get user home folder!");
            return Err(anyhow!(msg));
        }
    };
    Ok(home)
}

#[cfg(test)]
#[path = "./__tests__/get_user_home_dir.spec.rs"]
mod get_user_home_dir_test;