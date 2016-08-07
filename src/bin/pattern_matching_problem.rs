extern crate bio;

use bio::dna::u8::DNA;

/// Code Challenge: Solve the Pattern Matching Problem.
/// Input: Two strings, Pattern and Genome.
/// Output: A collection of space-separated integers specifying all starting
/// positions where Pattern appears as a substring of Genome.
fn main() {

    let mut pat_string = String::new();
    let mut dna_string = String::new();
    bio::io::read_line(&mut pat_string);
    bio::io::read_line(&mut dna_string);

    let pat = DNA::from_str(&pat_string);
    let dna = DNA::from_str(&dna_string);

    let res = bio::find(dna, pat);

    bio::io::print_vec(&res);
}
