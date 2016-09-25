extern crate bio;

use bio::seq::{find_by, hamming_distance};
use bio::dna::Dna;

/// Code Challenge: Implement ApproximatePatternCount.
/// Input: Strings Pattern and Text as well as an integer d.
/// Output: Countd(Text, Pattern).
fn main() {

    let mut pattern_string = String::new();
    let mut dna_string = String::new();
    let mut d_string = String::new();
    bio::io::read_line(&mut pattern_string);
    bio::io::read_line(&mut dna_string);
    bio::io::read_line(&mut d_string);

    let pattern = pattern_string.parse::<Dna>().unwrap();
    let dna = dna_string.parse::<Dna>().unwrap();
    let d = d_string.parse::<usize>().unwrap();

    let (indices, _) = find_by(&dna, &pattern, |chunk, pat| hamming_distance(chunk, pat) <= d);

    println!("{}", indices.len());
}
