use env_logger::Builder;
use log::LevelFilter;

/// Init env_logger with `verbose` CLI arg support
/// 
/// # Arguments
/// 
/// * `is_verbose` - true if the verbose CLI arg is set
pub fn init_logger(is_verbose: bool) {
    if is_verbose == false {
        env_logger::init();
    } else {
        Builder::new().filter_level(LevelFilter::max()).init();
    }
}

#[cfg(test)]
#[path = "./__tests__/init_logger.spec.rs"]
mod init_logger_test;
