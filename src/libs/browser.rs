use core::fmt;

#[derive(PartialEq, Debug)]
pub enum Browser {
    // Google Chrome
    Chrome,
    // Mozilla Firefox
    Firefox,
}

impl fmt::Display for Browser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Browser::Chrome => write!(f, "Chrome"),
            Browser::Firefox => write!(f, "Firefox"),
        }
    }
}
