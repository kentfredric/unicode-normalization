// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::char;
use super::UnicodeNormalization;
use super::char::is_combining_mark;


#[test]
fn test_nfd() {
    macro_rules! t {
        ($input: expr, $expected: expr) => {
            assert_eq!($input.nfd().to_string(), $expected);
            // A dummy iterator that is not std::str::Chars directly;
            // note that `id_func` is used to ensure `Clone` implementation
            assert_eq!($input.chars().map(|c| c).nfd().collect::<String>(), $expected);
        }
    }
    t!("abc", "abc");
    t!("\u{1e0b}\u{1c4}", "d\u{307}\u{1c4}");
    t!("\u{2026}", "\u{2026}");
    t!("\u{2126}", "\u{3a9}");
    t!("\u{1e0b}\u{323}", "d\u{323}\u{307}");
    t!("\u{1e0d}\u{307}", "d\u{323}\u{307}");
    t!("a\u{301}", "a\u{301}");
    t!("\u{301}a", "\u{301}a");
    t!("\u{d4db}", "\u{1111}\u{1171}\u{11b6}");
    t!("\u{ac1c}", "\u{1100}\u{1162}");
}

#[test]
fn test_nfkd() {
    macro_rules! t {
        ($input: expr, $expected: expr) => {
            assert_eq!($input.nfkd().to_string(), $expected);
        }
    }
    t!("abc", "abc");
    t!("\u{1e0b}\u{1c4}", "d\u{307}DZ\u{30c}");
    t!("\u{2026}", "...");
    t!("\u{2126}", "\u{3a9}");
    t!("\u{1e0b}\u{323}", "d\u{323}\u{307}");
    t!("\u{1e0d}\u{307}", "d\u{323}\u{307}");
    t!("a\u{301}", "a\u{301}");
    t!("\u{301}a", "\u{301}a");
    t!("\u{d4db}", "\u{1111}\u{1171}\u{11b6}");
    t!("\u{ac1c}", "\u{1100}\u{1162}");
}

#[test]
fn test_nfc() {
    macro_rules! t {
        ($input: expr, $expected: expr) => {
            assert_eq!($input.nfc().to_string(), $expected);
        }
    }
    t!("abc", "abc");
    t!("\u{1e0b}\u{1c4}", "\u{1e0b}\u{1c4}");
    t!("\u{2026}", "\u{2026}");
    t!("\u{2126}", "\u{3a9}");
    t!("\u{1e0b}\u{323}", "\u{1e0d}\u{307}");
    t!("\u{1e0d}\u{307}", "\u{1e0d}\u{307}");
    t!("a\u{301}", "\u{e1}");
    t!("\u{301}a", "\u{301}a");
    t!("\u{d4db}", "\u{d4db}");
    t!("\u{ac1c}", "\u{ac1c}");
    t!("a\u{300}\u{305}\u{315}\u{5ae}b", "\u{e0}\u{5ae}\u{305}\u{315}b");
}

#[test]
fn test_nfkc() {
    macro_rules! t {
        ($input: expr, $expected: expr) => {
            assert_eq!($input.nfkc().to_string(), $expected);
        }
    }
    t!("abc", "abc");
    t!("\u{1e0b}\u{1c4}", "\u{1e0b}D\u{17d}");
    t!("\u{2026}", "...");
    t!("\u{2126}", "\u{3a9}");
    t!("\u{1e0b}\u{323}", "\u{1e0d}\u{307}");
    t!("\u{1e0d}\u{307}", "\u{1e0d}\u{307}");
    t!("a\u{301}", "\u{e1}");
    t!("\u{301}a", "\u{301}a");
    t!("\u{d4db}", "\u{d4db}");
    t!("\u{ac1c}", "\u{ac1c}");
    t!("a\u{300}\u{305}\u{315}\u{5ae}b", "\u{e0}\u{5ae}\u{305}\u{315}b");
}

#[test]
fn test_official() {
    use normalization_tests::NORMALIZATION_TESTS;
    macro_rules! normString {
        ($method: ident, $input: expr) => { $input.$method().collect::<String>() }
    }

    for test in NORMALIZATION_TESTS {
        // these invariants come from the CONFORMANCE section of
        // http://www.unicode.org/Public/UNIDATA/NormalizationTest.txt
        {
            let r1 = normString!(nfc, test.source);
            let r2 = normString!(nfc, test.nfc);
            let r3 = normString!(nfc, test.nfd);
            let r4 = normString!(nfc, test.nfkc);
            let r5 = normString!(nfc, test.nfkd);
            assert_eq!(test.nfc, &r1[..]);
            assert_eq!(test.nfc, &r2[..]);
            assert_eq!(test.nfc, &r3[..]);
            assert_eq!(test.nfkc, &r4[..]);
            assert_eq!(test.nfkc, &r5[..]);
        }

        {
            let r1 = normString!(nfd, test.source);
            let r2 = normString!(nfd, test.nfc);
            let r3 = normString!(nfd, test.nfd);
            let r4 = normString!(nfd, test.nfkc);
            let r5 = normString!(nfd, test.nfkd);
            assert_eq!(test.nfd, &r1[..]);
            assert_eq!(test.nfd, &r2[..]);
            assert_eq!(test.nfd, &r3[..]);
            assert_eq!(test.nfkd, &r4[..]);
            assert_eq!(test.nfkd, &r5[..]);
        }

        {
            let r1 = normString!(nfkc, test.source);
            let r2 = normString!(nfkc, test.nfc);
            let r3 = normString!(nfkc, test.nfd);
            let r4 = normString!(nfkc, test.nfkc);
            let r5 = normString!(nfkc, test.nfkd);
            assert_eq!(test.nfkc, &r1[..]);
            assert_eq!(test.nfkc, &r2[..]);
            assert_eq!(test.nfkc, &r3[..]);
            assert_eq!(test.nfkc, &r4[..]);
            assert_eq!(test.nfkc, &r5[..]);
        }

        {
            let r1 = normString!(nfkd, test.source);
            let r2 = normString!(nfkd, test.nfc);
            let r3 = normString!(nfkd, test.nfd);
            let r4 = normString!(nfkd, test.nfkc);
            let r5 = normString!(nfkd, test.nfkd);
            assert_eq!(test.nfkd, &r1[..]);
            assert_eq!(test.nfkd, &r2[..]);
            assert_eq!(test.nfkd, &r3[..]);
            assert_eq!(test.nfkd, &r4[..]);
            assert_eq!(test.nfkd, &r5[..]);
        }
    }
}

#[test]
fn test_quick_check() {
    use normalization_tests::NORMALIZATION_TESTS;
    use quick_check;
    for test in NORMALIZATION_TESTS {
        assert!(quick_check::is_nfc(test.nfc));
        assert!(quick_check::is_nfd(test.nfd));
        assert!(quick_check::is_nfkc(test.nfkc));
        assert!(quick_check::is_nfkd(test.nfkd));
        if test.nfc != test.nfd {
            assert!(!quick_check::is_nfc(test.nfd));
            assert!(!quick_check::is_nfd(test.nfc));
        }
        if test.nfkc != test.nfc {
            assert!(!quick_check::is_nfkc(test.nfc));
            assert!(quick_check::is_nfc(test.nfkc));
        }
        if test.nfkd != test.nfd {
            assert!(!quick_check::is_nfkd(test.nfd));
            assert!(quick_check::is_nfd(test.nfkd));
        }
    }
}

#[test]
fn test_is_combining_mark_ascii() {
    for cp in 0..0x7f {
        assert!(!is_combining_mark(char::from_u32(cp).unwrap()));
    }
}

#[test]
fn test_is_combining_mark_misc() {
    // https://github.com/unicode-rs/unicode-normalization/issues/16
    // U+11C3A BHAIKSUKI VOWEL SIGN O
    // Category: Mark, Nonspacing [Mn]
    assert!(is_combining_mark('\u{11C3A}'));

    // U+11C3F BHAIKSUKI SIGN VIRAMA
    // Category: Mark, Nonspacing [Mn]
    assert!(is_combining_mark('\u{11C3F}'));
}
