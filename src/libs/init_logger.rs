use env_logger::Builder;
use log::LevelFilter;

/// Init env_logger with `verbose` CLI arg support
///
/// # Arguments
///
/// * `is_verbose` - true if the verbose CLI arg is set
pub fn init_logger(is_verbose: bool) -> String {
    if is_verbose == false {
        env_logger::init();
        return "env".to_string();
    } else {
        Builder::new().filter_level(LevelFilter::max()).init();
        return "verbose".to_string();
    }
}

#[cfg(test)]
#[path = "./__tests__/init_logger.spec.rs"]
mod init_logger_test;
