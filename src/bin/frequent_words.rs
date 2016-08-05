extern crate bio;

use std::io;

use bio::strings::pattern_count;

static FAILED_TO_READ_LINE: &'static str = "Failed to read line";

fn frequent_words(text: &str, k: usize) -> Vec<&str> {
    let len = text.len() - k + 1;
    let mut res = Vec::new();
    let mut count = vec![0; len];
    // compute counts
    for i in 0..len {
        let pat = &text[i..i+k];
        count[i] = pattern_count(text, pat);
    }
    // find maximum count
    let max_count = *count.iter().max().unwrap();
    // read patterns with maximum count
    for i in 0..len {
        if count[i] == max_count {
            res.push(&text[i..i+k]);
        }
    }
    // reduplicate and return
    res.sort();
    res.dedup();
    res
}

fn print_result(res: &[&str]) {
    for (i, r) in res.iter().enumerate() {
        if i < res.len() - 1 {
            print!("{} ", r);
        } else {
            print!("{}", r);
        }
    }
    println!("");
}

/// 1.2 Hidden Messages in the Replication Origin
///
/// Code Challenge: Solve the Frequent Words Problem.
/// Input: A string Text and an integer k.
/// Output: All most frequent k-mers in Text.
fn main() {

    let mut text = String::new();
    let mut kstr = String::new();
    io::stdin().read_line(&mut text)
        .expect(FAILED_TO_READ_LINE);
    io::stdin().read_line(&mut kstr)
        .expect(FAILED_TO_READ_LINE);

    let k = kstr.trim().parse::<usize>().unwrap();

    //println!("DEBUG: text={}", text.trim());
    //println!("DEBUG: k={}", k);

    let res = frequent_words(text.trim(), k);

    print_result(&res);
}
