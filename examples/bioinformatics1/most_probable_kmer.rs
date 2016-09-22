extern crate bio;

use std::env;
use std::str;

use bio::data::Dataset;
use bio::u8::{Dna, Profile, most_probable_kmer};

/// Profile-most Probable k-mer Problem: Find a Profile-most probable k-mer in a string.
/// Input: A string Text, an integer k, and a 4 × k matrix Profile.
/// Output: A Profile-most probable k-mer in Text.
fn main() {

    let file_name = env::args().nth(1).unwrap();

    let data = Dataset::open_text(file_name);
    let lines = data.lines();

    let dna = lines[0].parse::<Dna>().unwrap();
    let k = lines[1].parse::<usize>().unwrap();

    let pa = bio::io::parse_vec::<f64>(&lines[2]).unwrap();
    let pc = bio::io::parse_vec::<f64>(&lines[3]).unwrap();
    let pg = bio::io::parse_vec::<f64>(&lines[4]).unwrap();
    let pt = bio::io::parse_vec::<f64>(&lines[5]).unwrap();
    let profile = Profile::new(pa, pc, pg, pt);

    let (_, kmer) = most_probable_kmer(&dna, k, &profile);

    println!("{}", kmer);
}
