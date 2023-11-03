mod libs;
use clap::Parser;
use libs::{sanitize_browser_list, Args, Browser};

fn main() {
    env_logger::init();
    let args = Args::parse();

    if let Err(_) = sanitize_browser_list(args.browser, None) {
        std::process::exit(1)
    }
}
