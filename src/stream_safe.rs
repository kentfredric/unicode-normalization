use normalize::{
    hangul_decomposition_length,
    is_hangul_syllable,
};
use lookups::{
    canonical_combining_class, canonical_fully_decomposed, compatibility_fully_decomposed,
    stream_safe_trailing_nonstarters,
};
use tables::stream_safe_leading_nonstarters;

pub(crate) const MAX_NONSTARTERS: usize = 30;
const COMBINING_GRAPHEME_JOINER: char = '\u{034F}';

/// UAX15-D4: This iterator keeps track of how many non-starters there have been
/// since the last starter in *NFKD* and will emit a Combining Grapheme Joiner
/// (U+034F) if the count exceeds 30.
pub struct StreamSafe<I> {
    iter: I,
    nonstarter_count: usize,
    buffer: Option<char>,
}

impl<I> StreamSafe<I> {
    pub(crate) fn new(iter: I) -> Self {
        Self { iter, nonstarter_count: 0, buffer: None }
    }
}

impl<I: Iterator<Item=char>> Iterator for StreamSafe<I> {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        if let Some(ch) = self.buffer.take() {
            return Some(ch);
        }
        let next_ch = match self.iter.next() {
            None => return None,
            Some(c) => c,
        };
        let d = classify_nonstarters(next_ch);
        if self.nonstarter_count + d.leading_nonstarters > MAX_NONSTARTERS {
            self.buffer = Some(next_ch);
            self.nonstarter_count = 0;
            return Some(COMBINING_GRAPHEME_JOINER);
        }

        // No starters in the decomposition, so keep accumulating
        if d.leading_nonstarters == d.decomposition_len {
            self.nonstarter_count += d.decomposition_len;
        }
        // Otherwise, restart the nonstarter counter.
        else {
            self.nonstarter_count = d.trailing_nonstarters;
        }
        Some(next_ch)
    }
}

#[derive(Debug)]
pub(crate) struct Decomposition {
    pub(crate) leading_nonstarters: usize,
    pub(crate) trailing_nonstarters: usize,
    pub(crate) decomposition_len: usize,
}

#[inline]
pub(crate) fn classify_nonstarters(c: char) -> Decomposition {
    // As usual, fast path for ASCII (which is always a starter)
    if c <= '\x7f' {
        return Decomposition {
            leading_nonstarters: 0,
            trailing_nonstarters: 0,
            decomposition_len: 1,
        }
    }
    // Next, special case Hangul, since it's not handled by our tables.
    if is_hangul_syllable(c) {
        return Decomposition {
            leading_nonstarters: 0,
            trailing_nonstarters: 0,
            decomposition_len: hangul_decomposition_length(c),
        };
    }
    let decomp = compatibility_fully_decomposed(c)
        .or_else(|| canonical_fully_decomposed(c));
    match decomp {
        Some(decomp) => {
            Decomposition {
                leading_nonstarters: stream_safe_leading_nonstarters(c),
                trailing_nonstarters: stream_safe_trailing_nonstarters(c),
                decomposition_len: decomp.len(),
            }
        },
        None => {
            let is_nonstarter = canonical_combining_class(c) != 0;
            let nonstarter = if is_nonstarter { 1 } else { 0 };
            Decomposition {
                leading_nonstarters: nonstarter,
                trailing_nonstarters: nonstarter,
                decomposition_len: 1,
            }
        }
    }
}
