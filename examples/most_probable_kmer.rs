extern crate bio;

use std::env;
use std::str;

use bio::data::Dataset;
use bio::u8::{Dna, most_probable_kmer};

fn parse_vec<T: str::FromStr>(s: &str) -> Result<Vec<T>, T::Err> {
    s.split_whitespace()
        .fold(Ok(Vec::with_capacity(s.len())), |acc, c| {
            let mut vec = try!(acc);
            let x = try!(c.parse::<T>());
            vec.push(x);
            Ok(vec)
        })
}

/// Profile-most Probable k-mer Problem: Find a Profile-most probable k-mer in a string.
/// Input: A string Text, an integer k, and a 4 × k matrix Profile.
/// Output: A Profile-most probable k-mer in Text.
fn main() {

    let file_name = env::args().nth(1).unwrap();

    let data = Dataset::open_text(file_name);
    let lines = data.lines();

    let dna = lines[0].parse::<Dna>().unwrap();
    let k = lines[1].parse::<usize>().unwrap();

    let pa = parse_vec::<f64>(&lines[2]).unwrap();
    let pc = parse_vec::<f64>(&lines[3]).unwrap();
    let pg = parse_vec::<f64>(&lines[4]).unwrap();
    let pt = parse_vec::<f64>(&lines[5]).unwrap();

    let (_, kmer) = most_probable_kmer(&dna, k, &pa, &pc, &pg, &pt);

    println!("{}", kmer);
}
