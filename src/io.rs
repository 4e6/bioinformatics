use std::fmt::Display;
use std::io;
use std::str;

#[cfg(windows)] pub const NL: &'static str = "\r\n";
#[cfg(not(windows))] pub const NL: &'static str = "\n";

static FAILED_TO_READ_LINE: &'static str = "Failed to read line";

/// Reads a line from stdin, drops trailing newline.
pub fn read_line(s: &mut String) {
    io::stdin().read_line(s)
        .expect(FAILED_TO_READ_LINE);
    s.pop();
}

/// Parse string `s` as a whitespace separated vector of `T`.
pub fn parse_vec<T: str::FromStr>(s: &str) -> Result<Vec<T>, T::Err> {
    s.split_whitespace()
        .fold(Ok(Vec::with_capacity(s.len())), |acc, c| {
            let mut vec = try!(acc);
            let x = try!(c.parse::<T>());
            vec.push(x);
            Ok(vec)
        })
}

/// print vector contents in a line separated by space
pub fn print_vec<T: Display>(v: &[T]) {
    for (i, r) in v.iter().enumerate() {
        if i < v.len() - 1 {
            print!("{} ", r);
        } else {
            print!("{}", r);
        }
    }
    println!("");
}

pub fn println_vec<T: Display>(v: &[T]) {
    for r in v.iter() {
        println!("{}", r);
    }
}
