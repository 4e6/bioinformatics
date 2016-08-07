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

/// all distinct k-me rs in lexicographical order
pub fn kmers(text: &str, k: usize) -> Vec<&str> {
    let mut res = Vec::new();
    for i in 0..text.len()-k+1 {
        res.push(&text[i..i+k]);
    }
    res.sort();
    res.dedup();
    res
}

pub fn frequency_array(text: &str, k: usize) -> Vec<usize> {
    fn symbol_to_number(sym: u8) -> usize {
        match sym as char {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            c => panic!("symbol_to_number: invalid char {}", c)
        }
    }
    fn pattern_to_number(pat: &[u8]) -> usize {
        if pat.len() == 0 {
            0
        } else {
            let (init, last) = pat.split_at(pat.len()-1);
            4 * pattern_to_number(init) + symbol_to_number(last[0])
        }
    }
    let len = 4usize.pow(k as u32);
    let mut freqs = vec![0; len];
    for i in 0..text.len()-k+1 {
        let j = pattern_to_number(&text[i..i+k].as_bytes());
        freqs[j] += 1;
    }
    freqs
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
