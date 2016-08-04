extern crate util;

use std::io;
use util::pattern_count;

fn main() {
    let failed_to_read_line = "Failed to read line";

    let mut s = String::new();
    let mut k = String::new();
    io::stdin().read_line(&mut s)
        .expect(failed_to_read_line);
    io::stdin().read_line(&mut k)
        .expect(failed_to_read_line);

    let res = pattern_count(s.trim(), k.trim());
    println!("{}", res);
}
