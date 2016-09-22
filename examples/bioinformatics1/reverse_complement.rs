extern crate bio;

use bio::u8::Dna;

/// Reverse Complement Problem: Find the reverse complement of a Dna string.
/// Input: A Dna string Pattern.
/// Output: Pattern, the reverse complement of Pattern.
fn main() {

    let mut dna_string = String::new();
    bio::io::read_line(&mut dna_string);

    let dna = Dna::from_str_unchecked(&dna_string);
    let dna_rcomp = dna.reverse_complement();

    println!("{}", dna_rcomp);
}
