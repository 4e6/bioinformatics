extern crate bio;

use bio::strings::frequent_words;

/// 1.2 Hidden Messages in the Replication Origin
///
/// Code Challenge: Solve the Frequent Words Problem.
/// Input: A string Text and an integer k.
/// Output: All most frequent k-mers in Text.
fn main() {

    let mut text = String::new();
    let mut kstr = String::new();
    bio::io::read_line(&mut text);
    bio::io::read_line(&mut kstr);

    let k = kstr.parse::<usize>().unwrap();

    //println!("DEBUG: text={}", text.trim());
    //println!("DEBUG: k={}", k);

    let res = frequent_words(&text, k);

    bio::io::print_vec(&res);
}
