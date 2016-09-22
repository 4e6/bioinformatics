extern crate bio;

use bio::strings::pattern_to_number;

fn main() {

    let mut pattern = String::new();
    bio::io::read_line(&mut pattern);

    let res = pattern_to_number(pattern.as_bytes());

    println!("{}", res);
}
