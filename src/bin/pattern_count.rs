extern crate bio;

use std::io;

use bio::strings::pattern_count;

static FAILED_TO_READ_LINE: &'static str = "Failed to read line";

/// 1.2 Hidden Messages in the Replication Origin
///
/// Code Challenge: Implement PatternCount.
/// Input: Strings Text and Pattern.
/// Output: Count(Text, Pattern).
fn main() {

    let mut text = String::new();
    let mut pat = String::new();
    io::stdin().read_line(&mut text)
        .expect(FAILED_TO_READ_LINE);
    io::stdin().read_line(&mut pat)
        .expect(FAILED_TO_READ_LINE);

    let res = pattern_count(text.trim(), pat.trim());
    println!("{}", res);
}
