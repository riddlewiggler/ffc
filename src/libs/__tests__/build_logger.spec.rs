#[cfg(test)]
use super::build_logger;
use anyhow::Result;

#[test]
fn it_should_return_a_builder_if_arg_is_true() -> Result<()> {
    let ret = build_logger(true);
    assert!(ret.is_ok());
    Ok(())
}

#[test]
fn it_should_return_a_builder_if_arg_is_false() -> Result<()> {
    let ret = build_logger(false);
    assert!(ret.is_ok());
    Ok(())
}
