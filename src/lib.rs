#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![doc = include_str!("../README.md")]

mod consts;

/// Converts a number to base64 in O(log(n)) time.
///
/// # Examples
///
/// ```
/// use conv64::encode;
///
/// assert_eq!(encode(0), "0");
/// assert_eq!(encode(1), "1");
/// assert_eq!(encode(145), "2H");
/// ```
pub fn encode(mut num: usize) -> String {
	if num < 64 {
		return consts::CHARS[num].into();
	}

	let estimated_length = num.next_power_of_two().trailing_zeros().max(1);
	let mut b64 = String::with_capacity(estimated_length as usize);

	while num > 0 {
		// `num & 63`` is equivalent to `num % 64`,
		// and `num >> 6` is equivalent to `num / 64`.
		// Bitwise operations are usually faster, so we use them instead.
		b64.insert(0, consts::CHARS[num & 63]);
		num >>= 6;
	}

	b64
}

/// Generates a random base 64 number between a range of base 10 numbers.
///
/// # Examples
///
/// ```
/// use conv64::generate;
///
/// let id = generate(0..100);
/// assert!(id.len() <= 2);
/// ```
#[cfg(feature = "rand")]
pub fn generate(range: std::ops::Range<usize>) -> String {
	use rand::Rng;

	let mut rng = rand::thread_rng();
	let num = rng.gen_range(range);

	encode(num)
}

/// Gets the max value a base64 string with `n` chars can represent.
///
/// Equivalent to 64<sup>n</sup> - 1.
///
/// Returns `None` if the value is too big to fit in a `usize`.
///
/// # Examples
/// ```
/// use conv64::max_value_for_letters;
///
/// assert_eq!(max_value_for_letters(1), Some(63));
/// assert_eq!(max_value_for_letters(2), Some(4095));
/// ```
pub fn max_value_for_letters(n: u32) -> Option<usize> {
	64usize.checked_pow(n).map(|x| x - 1)
}

#[cfg(test)]
mod tests {
	use crate::{consts::CHARS, encode};

	#[test]
	fn test_to_b64() {
		for i in 0..=9 {
			assert_eq!(encode(i), i.to_string());
		}

		for (i, c) in CHARS.iter().enumerate().skip(9) {
			assert_eq!(encode(i), c.to_string());
			assert_eq!(encode(i).len(), 1);
		}

		assert_eq!(encode(64), "10");
		assert_eq!(encode(65), "11");

		assert_eq!(encode(64 * 2), "20");
		assert_eq!(encode(64 * 2 + 1), "21");
	}

	#[test]
	#[cfg(feature = "rand")]
	fn generate() {
		for _ in 0..100 {
			let id = crate::generate(0..100);
			assert!(id.len() <= 2);
		}
	}
}
