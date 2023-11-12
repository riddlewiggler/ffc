#[cfg(test)]
use super::init_logger;

#[test]
fn it_should_return_verbose_if_arg_is_true() {
    let ret = init_logger(true);
    assert_eq!(ret, "verbose".to_string());
}

#[test]
fn it_should_return_env_if_arg_is_false() {
    let ret = init_logger(false);
    assert_eq!(ret, "env".to_string());
}
