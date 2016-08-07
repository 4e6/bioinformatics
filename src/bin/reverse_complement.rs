extern crate bio;

use bio::dna::u8::DNA;

/// Reverse Complement Problem: Find the reverse complement of a DNA string.
/// Input: A DNA string Pattern.
/// Output: Pattern, the reverse complement of Pattern.
fn main() {

    let mut dna_string = String::new();
    bio::io::read_line(&mut dna_string);

    let dna = DNA::from_str(&dna_string);
    let complement = dna.reverse_complement();

    println!("{}", complement.to_string());
}
