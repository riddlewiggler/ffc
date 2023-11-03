mod files;

mod sanitize_browser;
pub use sanitize_browser::sanitize_browser;

mod browser;
pub use browser::Browser;

mod cli_args;
pub use cli_args::{Args, get_styles};

mod sanitize_browser_list;
pub use sanitize_browser_list::sanitize_browser_list;