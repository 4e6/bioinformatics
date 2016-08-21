extern crate bio;

use std::env;

use bio::data::Dataset;
use bio::u8::{Dna, motif_enumeration};

/// Code Challenge: Implement MotifEnumeration (reproduced below).
/// Input: Integers k and d, followed by a collection of strings Dna.
/// Output: All (k, d)-motifs in Dna.
fn main() {

    let filename = env::args().nth(1).unwrap();

    let data = Dataset::open_text(filename);
    let lines = data.lines();
    let (hd, tl) = lines.split_first().unwrap();
    let kd: Vec<usize> = hd
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let dnas: Vec<Dna> = tl.iter()
        .map(|x| Dna::from_str_unchecked(x))
        .collect();

    let motifs = motif_enumeration(&dnas, kd[0], kd[1]);
    let mut res: Vec<Dna> = motifs.into_iter().collect();
    res.sort();
    bio::io::print_vec(&res);
}
