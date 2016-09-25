extern crate bio;

use bio::seq;
use bio::dna::Dna;

/// Approximate Pattern Matching Problem: Find all approximate occurrences of a pattern in a string.
/// Input: Strings Pattern and Text along with an integer d.
/// Output: All starting positions where Pattern appears as a substring of Text with at most d mismatches.
fn main() {

    let mut pattern_string = String::new();
    let mut dna_string = String::new();
    let mut d_string = String::new();
    bio::io::read_line(&mut pattern_string);
    bio::io::read_line(&mut dna_string);
    bio::io::read_line(&mut d_string);

    let pattern = Dna::from_string(pattern_string);
    let dna = Dna::from_string(dna_string);
    let d = d_string.parse::<usize>().unwrap();

    let (indices, _) = seq::find_by(&dna, &pattern, |chunk, pat| seq::hamming_distance(chunk, pat) <= d);

    bio::io::print_vec(&indices);
}
