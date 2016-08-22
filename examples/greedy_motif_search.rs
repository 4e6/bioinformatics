extern crate bio;

use std::env;

use bio::data::Dataset;
use bio::u8::{Dna, greedy_motif_search};

/// Code Challenge: Implement GreedyMotifSearch.
/// Input: Integers k and t, followed by a collection of strings Dna.
/// Output: A collection of strings BestMotifs resulting from applying GreedyMotifSearch(Dna, k, t).
/// If at any step you find more than one Profile-most probable k-mer in a given string, use the
/// one occurring first.
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

    let motifs = greedy_motif_search(&dnas, kt[0], kt[1]);
    let res: Vec<_> = motifs.iter().map(|x| Dna::from_slice(x)).collect();
    bio::io::println_vec(&res);
}
