use clap::Parser;

const ABOUT: &str = r#"
Fun with Forty Client (ffc for brevity) cleans the certificate authority files
(pkcs11.txt) used by your browsers and polluted by Forty Client"#;

/// Fun with Forty Client
#[derive(Parser, Debug)]
#[command(author, version, about = ABOUT, long_about = None, styles=get_styles())]
pub struct Args {
    /// Browser[s] to clean
    #[arg(short, long, num_args = 0..3, value_delimiter = ' ', default_values_t = ["chrome".to_string(), "firefox".to_string()])]
    pub browser: Vec<String>,
}

/// Generate the styles for helper output of clap
/// 
/// [info](https://stackoverflow.com/a/76916424/2816883)
pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}


#[cfg(test)]
#[path = "./__tests__/cli_args.spec.rs"]
mod cli_args_test;
