extern crate bio;

use std::env;

use bio::data::Dataset;
use bio::u8::{Dna, gibbs_sampler};

const ITERATIONS: usize = 300;

/// Code Challenge: Implement GibbsSampler.
/// Input: Integers k, t, and N, followed by a collection of strings Dna.
/// Output: The strings BestMotifs resulting from running GibbsSampler(Dna, k, t, N) with
/// 20 random starts. Remember to use pseudocounts!
fn main() {

    let file_name = env::args().nth(1).unwrap();

    let data = Dataset::open_text(file_name);
    let lines = data.lines();
    let ktn: Vec<_> = lines[0]
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let dnas: Vec<_> = lines[1..].iter()
        .map(|x| Dna::from_str_unchecked(x))
        .collect();

    assert_eq!(dnas.len(), ktn[1]);
    let motifs = gibbs_sampler(&dnas, ktn[0], ktn[1], ktn[2], ITERATIONS);
    bio::io::println_vec(&motifs);
}
