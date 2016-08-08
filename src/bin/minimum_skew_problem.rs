extern crate bio;

use bio::strings::{skew, skew_min};

/// Minimum Skew Problem: Find a position in a genome where the skew diagram attains a minimum.
/// Input: A DNA string Genome.
/// Output: All integer(s) i minimizing Skewi (Genome) among all values of i (from 0 to |Genome|).
fn main() {

    let mut genome = String::new();
    bio::io::read_line(&mut genome);

    let skew = skew(&genome);
    let mut min = skew_min(&skew);
    // grading program expect 1-based indexing
    for e in min.iter_mut() {
        *e += 1;
    }

    bio::io::print_vec(&min);
}
