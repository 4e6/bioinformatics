extern crate bio;

use bio::strings::number_to_pattern;

fn main() {

    let mut index_string = String::new();
    let mut k_string = String::new();
    bio::io::read_line(&mut index_string);
    bio::io::read_line(&mut k_string);

    let index = index_string.parse::<usize>().unwrap();
    let k = k_string.parse::<usize>().unwrap();

    let res = number_to_pattern(index, k);

    println!("{}", res);
}
