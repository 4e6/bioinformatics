extern crate bio;

use bio::strings::pattern_count;

/// 1.2 Hidden Messages in the Replication Origin
///
/// Code Challenge: Implement PatternCount.
/// Input: Strings Text and Pattern.
/// Output: Count(Text, Pattern).
fn main() {

    let mut text = String::new();
    let mut pat = String::new();
    bio::io::read_line(&mut text);
    bio::io::read_line(&mut pat);

    let res = pattern_count(&text, &pat);
    println!("{}", res);
}
