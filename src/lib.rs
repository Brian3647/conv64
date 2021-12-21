//! Utility for working with IDs using Base64
//!
//! # Why base64 for IDs?
//!
//! I recommend taking a look to this Tom scott video: https://www.youtube.com/watch?v=gocwRvLhDf8

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod consts;

pub use consts::*;

use rand::Rng;
use std::ops::Range;

/// Converts a number to a base64 string.
///
/// # Examples
///
/// ```
/// use b64id::to_b64;
/// use b64id::CHARS;
///
/// for i in 0..=9 {
///     assert_eq!(to_b64(i), i.to_string());
/// }
///
/// for (i, c) in CHARS.iter().enumerate().skip(9) {
///     assert_eq!(to_b64(i), c.to_string());
/// }
/// ```
pub fn to_b64(num: usize) -> String {
    let mut num = num;
    let mut b64 = String::new();

    if num == 0 {
        return "0".into();
    }

    while num > 0 {
        b64 += &CHARS[(num & 63)].to_string();
        num >>= 6;
    }

    b64.chars().rev().collect()
}

/// Generates a random ID with range.
pub fn generate(range: Range<usize>) -> String {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(range);

    to_b64(num)
}

/// Gets the max value a base64 string with \<number_of_letters\> chars can represent.
///
/// Returns 0 if the number of letters is bigger than 10. (if not checked and number is bigger than 10,
/// it will panic with overflow)
pub fn max_value_for_letters(number_of_letters: u8) -> usize {
    if number_of_letters > 10 {
        0
    } else {
        64_usize.pow(number_of_letters as u32)
    }
}

#[cfg(test)]
mod tests {
    use crate::{generate, to_b64, CHARS};

    #[test]
    fn test_to_b64() {
        for i in 0..=9 {
            assert_eq!(to_b64(i), i.to_string());
        }

        for (i, c) in CHARS.iter().enumerate().skip(9) {
            assert_eq!(to_b64(i), c.to_string());
            assert_eq!(to_b64(i).len(), 1);
        }

        println!("{:?}", dbg!(1));

        assert_eq!(to_b64(65), "11".to_string());
        assert_eq!(to_b64(66), "12".to_string());
        assert_eq!(to_b64(74), "1A".to_string());
        assert_eq!(to_b64(75), "1B".to_string());
        assert_eq!(to_b64(100), "1a".to_string());
        assert_eq!(to_b64(126), "1-".to_string());
        assert_eq!(to_b64(127), "1_".to_string());
        assert_eq!(to_b64(128), "20".to_string());
        assert_eq!(to_b64(164), "2a".to_string());
    }

    #[test]
    fn test_gen() {
        for i in 1000..2000 {
            generate(i..usize::MAX);
        }
    }
}
