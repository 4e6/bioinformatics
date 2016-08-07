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

pub fn pattern_count(text: &str, pat: &str) -> usize {
    self::indexes(text, pat).len()
}

fn frequent_words_count(text: &str, k: usize) -> Vec<usize> {
    let len = text.len() - k + 1;
    let mut count = vec![0; len];
    // compute counts
    for i in 0..len {
        let pat = &text[i..i+k];
        count[i] = self::pattern_count(text, pat);
    }
    count
}

pub fn frequent_words(text: &str, k: usize) -> Vec<&str> {
    let mut res = Vec::new();
    let count = frequent_words_count(text, k);
    // find maximum count
    let max_count = *count.iter().max().unwrap();
    // read patterns with maximum count
    for i in 0..count.len() {
        if count[i] == max_count {
            res.push(&text[i..i+k]);
        }
    }
    // reduplicate and return
    res.sort();
    res.dedup();
    res
}

pub fn clump_finding_naive(text: &str, k: usize, l: usize, t: usize) -> Vec<&str> {
    let len = text.len() - l + 1;
    let mut res = Vec::new();
    for i in 0..len {
        let chunk = &text[i..i+l];
        let count = self::frequent_words_count(chunk, k);

        for j in 0..count.len() {
            if count[j] == t {
                res.push(&chunk[j..j+k]);
            }
        }
    }
    res.sort();
    res.dedup();
    res
}
