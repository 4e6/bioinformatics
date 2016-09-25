extern crate bio;

use std::str;
use bio::dna::Dna;

/// Code Challenge: Solve the String Composition Problem.
/// Input: An integer k and a string Text.
/// Output: Compositionk(Text) (the k-mers can be provided in any order).
fn main() {

    let mut s_dna = String::new();
    let mut s_k = String::new();

    bio::io::read_line(&mut s_k);
    bio::io::read_line(&mut s_dna);

    let k = s_k.parse::<usize>().unwrap();
    let dna = Dna::from_string(s_dna);

    let mut res = dna.windows(k)
        .map(|x| unsafe { str::from_utf8_unchecked(x) })
        .collect::<Vec<_>>();
    res.sort();

    bio::io::println_vec(&res);
}
