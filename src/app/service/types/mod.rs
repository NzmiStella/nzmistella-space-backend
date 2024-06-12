// ********************* mod ********************* //
pub mod user;

pub mod prelude {
    pub use super::user::prelude::*;
}

// ********************* import ********************* //
use once_cell::sync::Lazy;
use regex::Regex;

// ********************* content ********************* //
static BASIC_ASCII_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?-u:\w)+$").unwrap());
static BASIC_UNICODE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\w+$").unwrap());
