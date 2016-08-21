extern crate bio;

use std::env;

use bio::data::Dataset;
use bio::u8::{Dna, median_string};

/// Code Challenge: Implement MedianString.
/// Input: An integer k, followed by a collection of strings Dna.
/// Output: A k-mer Pattern that minimizes d(Pattern, Dna) among all k-mers Pattern.
/// (If there are multiple such strings Pattern, then you may return any one.)
fn main() {

    let file_name = env::args().nth(1).unwrap();

    let data = Dataset::open_text(file_name);
    let lines = data.lines();

    let k = lines[0].parse::<usize>().unwrap();
    let dnas: Vec<Dna> = lines[1..].into_iter()
        .map(|x| Dna::from_str_unchecked(x))
        .collect();

    let median = median_string(&dnas, k);

    println!("{}", median);
}
