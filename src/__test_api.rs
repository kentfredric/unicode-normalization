// This crate comprises hacks and glue required to test private functions from tests/
//
// Keep this as slim as possible.
//
// If you're caught using this outside this crates tests/, you get to clean up the mess.

pub use crate::stream_safe::StreamSafe;
use crate::stream_safe;
use crate::lookups;
use crate::normalize;

pub use crate::lookups::canonical_combining_class;
pub use crate::normalize::decompose_compatible;

pub struct Decomposition {
    pub leading_nonstarters: usize,
    pub trailing_nonstarters: usize,
    pub decomposition_len: usize,
}
impl Into<Decomposition> for stream_safe::Decomposition {
    fn into (self: stream_safe::Decomposition) -> Decomposition { 
            Decomposition {
                leading_nonstarters: self.leading_nonstarters,
                trailing_nonstarters: self.trailing_nonstarters,
                decomposition_len: self.decomposition_len,
            }
    }
}

pub fn classify_nonstarters(c: char) -> Decomposition {
    stream_safe::classify_nonstarters(c).into()
}
pub fn stream_safe(s: &str) -> String {
        StreamSafe::new(s.chars()).collect()
}

pub fn compose_hangul(a: char, b: char) -> Option<char> {
    normalize::compose_hangul(a,b)
}

pub mod quick_check {
    pub use crate::quick_check::*;
}
