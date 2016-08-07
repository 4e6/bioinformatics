extern crate bio;

use std::io;

use bio::dna::u8::DNA;

static FAILED_TO_READ_LINE: &'static str = "Failed to read line";

fn main() {

    let mut dna_string = String::new();
    io::stdin().read_line(&mut dna_string)
        .expect(FAILED_TO_READ_LINE);

    let dna = DNA::from_str(dna_string.trim());
    let mut complement = dna.complement();
    complement.reverse();

    println!("{}", complement.to_string());
}
