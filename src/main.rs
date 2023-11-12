mod libs;
use clap::Parser;
use libs::{init_logger, sanitize_browser_list, Args, Browser};

fn main() {
    let args = Args::parse();
    init_logger(args.verbose);

    if let Err(_) = sanitize_browser_list(args.browser, None) {
        std::process::exit(1)
    }
}
