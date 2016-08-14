extern crate bio;

use bio::strings::hamming_distance;

/// Hamming Distance Problem: Compute the Hamming distance between two strings.
/// Input: Two strings of equal length.
/// Output: The Hamming distance between these strings.
fn main() {

    let mut xs = String::new();
    let mut ys = String::new();
    bio::io::read_line(&mut xs);
    bio::io::read_line(&mut ys);

    let hd = hamming_distance(&xs, &ys);

    println!("{}", hd);
}
