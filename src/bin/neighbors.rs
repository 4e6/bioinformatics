extern crate bio;

use bio::strings::neighbors;

/// Code Challenge: Implement Neighbors to find the d-neighborhood of a string.
/// Input: A string Pattern and an integer d.
/// Output: The collection of strings Neighbors(Pattern, d).
/// (You may return the strings in any order, but each line should contain only one string.)
fn main() {

    let mut pattern = String::new();
    let mut d_string = String::new();
    bio::io::read_line(&mut pattern);
    bio::io::read_line(&mut d_string);
    let d = d_string.parse::<usize>().unwrap();

    let mut res = neighbors(&pattern, d);
    // sort to be able to compare with saved result
    res.sort();

    bio::io::println_vec(&res);
}
