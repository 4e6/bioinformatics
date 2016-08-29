extern crate bio;

use std::env;

use bio::data::Dataset;
use bio::u8::{Dna, randomized_motif_search};

/// Code Challenge: Implement RandomizedMotifSearch.
/// Input: Integers k and t, followed by a collection of strings Dna.
/// Output: A collection BestMotifs resulting from running RandomizedMotifSearch(Dna, k, t) 1,000
/// times. Remember to use pseudocounts!
fn main() {

    let file_name = env::args().nth(1).unwrap();

    let data = Dataset::open_text(file_name);
    let lines = data.lines();
    let kt: Vec<_> = lines[0]
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let dnas: Vec<_> = lines[1..].iter()
        .map(|x| Dna::from_str_unchecked(x))
        .collect();

    assert_eq!(dnas.len(), kt[1]);
    let motifs = randomized_motif_search(&dnas, kt[0], 1000);
    bio::io::println_vec(&motifs);
}
