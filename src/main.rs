mod libs;
use clap::Parser;
use libs::{build_logger, sanitize_browser_list, Args, Browser};

fn main() {
    let args = Args::parse();
    build_logger(args.verbose).init();

    if let Err(_) = sanitize_browser_list(args.browser, None) {
        std::process::exit(1)
    }
}
