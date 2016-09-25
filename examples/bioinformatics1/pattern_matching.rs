extern crate bio;

use bio::seq::find;

/// Code Challenge: Solve the Pattern Matching Problem.
/// Input: Two strings, Pattern and Genome.
/// Output: A collection of space-separated integers specifying all starting
/// positions where Pattern appears as a substring of Genome.
fn main() {

    let mut pat = String::new();
    let mut dna = String::new();
    bio::io::read_line(&mut pat);
    bio::io::read_line(&mut dna);

    let inds = find(dna.as_bytes(), pat.as_bytes());

    bio::io::print_vec(&inds);
}
