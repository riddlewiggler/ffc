#[cfg(test)]
use super::build_logger;
use anyhow::Result;

#[test]
fn it_should_return_a_builder_if_arg_is_true() -> Result<()> {
    let _ = build_logger(true);
    Ok(())
}

#[test]
fn it_should_return_a_builder_if_arg_is_false() -> Result<()> {
    let _ = build_logger(false);
    Ok(())
}
