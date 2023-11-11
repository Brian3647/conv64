# conv64

Easily (& blazingly fast) convert base 10 numbers to base 64.

**IMPORTANT NOTE: This is base 64 as in the number base, if you're searching for base64 encryption, use [this](https://crates.io/crates/base64)**

## Quickstart

```toml
# Cargo.toml
[dependencies]
conv64 = "*"
```

```rust
// main.rs
fn main() {
    let x = conv64::encode(1234567890);
    println!("{}", x); // 19bWBI

    // With the `rand` feature enabled:
    let y = conv64::generate(1..2000);
    println!("{}", y); // A random number between 1 and 2000, encoded in base 64

    // Get the maximum value a base 64 number can have with a given number of letters
    assert_eq!(conv64::max_value_for_letters(1), Some(63));
}
```

## Uses

The main application for this is using it as an ID generator/shortener (for example, YouTube video IDs are base 64 encoded numbers).

Since its main use is for the web, the 2 extra characters (`+` and `/`) are replaced with `-` and `_` respectively, so it can be used in URLs without encoding.

## License

This project is licensed under the [MIT License](LICENSE).
