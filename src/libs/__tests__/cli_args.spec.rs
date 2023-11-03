#[cfg(test)]
use super::get_styles;
use anyhow::Result;

#[test]
fn get_styles_should_an_object() -> Result<()> {
    let styles = get_styles();

    let mut prop = styles.get_header();
    let mut fg_color = prop.get_fg_color().unwrap();
    assert_eq!(fg_color, anstyle::Color::Ansi(anstyle::AnsiColor::Yellow));

    prop = styles.get_error();
    fg_color = prop.get_fg_color().unwrap();
    assert_eq!(fg_color, anstyle::Color::Ansi(anstyle::AnsiColor::Red));

    Ok(())
}
