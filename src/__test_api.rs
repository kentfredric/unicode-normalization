pub use crate::stream_safe::StreamSafe;
use crate::stream_safe;
use crate::lookups;
pub use crate::lookups::canonical_combining_class;
pub use crate::normalize::decompose_compatible;
pub use crate::normalization_tests::NORMALIZATION_TESTS;

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

pub mod quick_check {
    pub use crate::quick_check::*;
}
