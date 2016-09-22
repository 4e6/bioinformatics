extern crate bio;

use bio::strings::frequent_words_with_mismatches;


/// Frequent Words with Mismatches Problem: Find the most frequent k-mers with mismatches in a string.
/// Input: A string Text as well as integers k and d. (You may assume k ≤ 12 and d ≤ 3.)
/// Output: All most frequent k-mers with up to d mismatches in Text.
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

    let res = frequent_words_with_mismatches(&text, k, d);

    bio::io::print_vec(&res);
}
