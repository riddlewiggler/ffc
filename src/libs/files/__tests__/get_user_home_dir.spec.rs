use super::get_user_home_dir;
use anyhow::Result;
#[cfg(test)]
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[test]
fn it_should_get_user_home_dir() -> Result<()> {
    let ret = get_user_home_dir()?;
    assert!(type_of(&ret).contains(&"String"));
    Ok(())
}
