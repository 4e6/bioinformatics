extern crate bioinformatics;

use std::io;

use bioinformatics::{ DNA, find };

static FAILED_TO_READ_LINE: &'static str = "Failed to read line";

fn main() {

    let mut s = String::new();
    let mut k = String::new();
    io::stdin().read_line(&mut s)
        .expect(FAILED_TO_READ_LINE);
    io::stdin().read_line(&mut k)
        .expect(FAILED_TO_READ_LINE);

    //let res = pattern_count(s.trim(), k.trim());
    let indexes = find(DNA::from_str(&s.trim()), DNA::from_str(&k.trim()));
    let res = indexes.len();
    println!("{}", res);
}
