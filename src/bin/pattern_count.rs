extern crate bio;

use std::io;

use bio::strings::indexes;

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

    let inds = indexes(text.trim(), pat.trim());
    let res = inds.len();
    println!("{}", res);
}
