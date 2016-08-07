extern crate bio;

use bio::strings::clump_finding;

/// Clump Finding Problem: Find patterns forming clumps in a string.
/// Input: A string Genome, and integers k, L, and t.
/// Output: All distinct k-mers forming (L, t)-clumps in Genome.
fn main() {

    let mut genome = String::new();
    let mut params = String::new();
    bio::io::read_line(&mut genome);
    bio::io::read_line(&mut params);

    let mut v: [usize; 3] = [0; 3];

    for (i, p) in params.split_whitespace().enumerate() {
        v[i] = p.parse::<usize>().unwrap();
    };

    let k = v[0];
    let l = v[1];
    let t = v[2];

    let res = clump_finding(&genome, k, l, t);

    bio::io::print_vec(&res);
}
