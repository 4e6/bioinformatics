extern crate bio;

use std::str::from_utf8_unchecked;
use bio::u8::Dna;

/// Code Challenge: Solve the String Composition Problem.
/// Input: An integer k and a string Text.
/// Output: Compositionk(Text) (the k-mers can be provided in any order).
fn main() {

    let mut s_dna = String::new();
    let mut s_k = String::new();

    bio::io::read_line(&mut s_k);
    bio::io::read_line(&mut s_dna);

    let k = s_k.parse::<usize>().unwrap();
    let dna = Dna::from_str_unchecked(&s_dna);

    let mut res = dna.windows(k)
        .map(|x| unsafe { from_utf8_unchecked(x) })
        .collect::<Vec<_>>();
    res.sort();

    bio::io::println_vec(&res);
}
