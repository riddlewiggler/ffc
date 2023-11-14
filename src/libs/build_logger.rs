use env_logger::Builder;
use log::LevelFilter;

/// Build env_logger::Builder with `verbose` CLI arg support
///
/// # Arguments
///
/// * `is_verbose` - true if the verbose CLI arg is set
pub fn build_logger(is_verbose: bool) -> Builder {
    let mut builder: Builder;
    if is_verbose == false {
        builder = Builder::from_default_env();
    } else {
        builder = Builder::new();
        builder.filter_level(LevelFilter::max());
    }

    builder
}

#[cfg(test)]
#[path = "./__tests__/build_logger.spec.rs"]
mod build_logger_test;
