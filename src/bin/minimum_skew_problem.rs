extern crate bio;

use bio::strings::{gc_skew, min_indices};

/// Minimum Skew Problem: Find a position in a genome where the skew diagram attains a minimum.
/// Input: A DNA string Genome.
/// Output: All integer(s) i minimizing Skewi (Genome) among all values of i (from 0 to |Genome|).
fn main() {

    let mut genome = String::new();
    bio::io::read_line(&mut genome);

    let skew = gc_skew(&genome);
    let (_, mut inds) = min_indices(skew);
    // grading program expect 1-based indexing
    for i in inds.iter_mut() {
        *i += 1;
    }

    bio::io::print_vec(&inds);
}
