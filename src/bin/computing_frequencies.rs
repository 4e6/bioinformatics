extern crate bio;

use bio::strings::frequency_array;

/// Code Challenge: Implement ComputingFrequencies to generate a frequency array.
/// Input: A DNA string Text followed by an integer k.
/// Output: FrequencyArray(Text).
fn main() {

    let mut text = String::new();
    let mut kstr = String::new();
    bio::io::read_line(&mut text);
    bio::io::read_line(&mut kstr);
    let k = kstr.parse::<usize>().unwrap();

    let res = frequency_array(&text, k);

    bio::io::print_vec(&res);
}
