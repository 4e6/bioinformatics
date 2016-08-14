extern crate bio;

use bio::strings::frequent_words_with_mismatches_and_reverse_complements;

/// Frequent Words with Mismatches and Reverse Complements Problem: Find the most frequent k-mers (with mismatches and reverse complements) in a string.
/// Input: A DNA string Text as well as integers k and d.
/// Output: All k-mers Pattern maximizing the sum Countd(Text, Pattern)+ Countd(Text, Pattern)
/// over all possible k-mers
fn main() {

    let mut text = String::new();
    let mut ints = String::new();
    bio::io::read_line(&mut text);
    bio::io::read_line(&mut ints);

    let mut k: usize = 0;
    let mut d: usize = 0;

    for (i, x) in ints.split_whitespace().enumerate() {
        match i {
            0 => k = x.parse::<usize>().unwrap(),
            1 => d = x.parse::<usize>().unwrap(),
            _ => break,
        }
    }

    let res = frequent_words_with_mismatches_and_reverse_complements(&text, k, d);

    bio::io::print_vec(&res);
}
