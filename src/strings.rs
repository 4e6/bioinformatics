//! Utilities to work with strings
//!

/// Returns byte indexes of the first character of this string slice that
/// matches the pattern.
///
/// # Panics
/// All characters should be `u8` ASCII
pub fn indexes(text: &str, pat: &str) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 0..text.len()-pat.len()+1 {
        if &text[i..i+pat.len()] == pat {
            res.push(i);
        }
    }
    res
}
