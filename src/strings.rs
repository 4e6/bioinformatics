//! Algorithms from Bioinformatics course.
//!
//! Algorithms from the course usually described in terms of arrays and indexed
//! access. Modules `u8` and `adt` contains proper implementation.

use std::collections::HashSet;
use std::iter::Scan;
use std::str::Chars;

static A: &'static str = "A";
static T: &'static str = "T";
static G: &'static str = "G";
static C: &'static str = "C";

/// Returns byte indexes of the first character of this string slice that
/// matches the pattern.
///
/// # Panics
///
/// All characters should be `u8` ASCII
fn indexes(text: &str, pat: &str) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 0..text.len()-pat.len()+1 {
        if &text[i..i+pat.len()] == pat {
            res.push(i);
        }
    }
    res
}

fn pattern_count(text: &str, pat: &str) -> usize {
    self::indexes(text, pat).len()
}

fn frequent_words_count(text: &str, k: usize) -> Vec<usize> {
    let len = text.len() - k + 1;
    let mut count = vec![0; len];
    // compute counts
    for i in 0..len {
        let pat = &text[i..i+k];
        count[i] = self::pattern_count(text, pat);
    }
    count
}

pub fn frequent_words(text: &str, k: usize) -> Vec<&str> {
    let mut res = Vec::new();
    let count = frequent_words_count(text, k);
    // find maximum count
    let max_count = *count.iter().max().unwrap();
    // read patterns with maximum count
    for i in 0..count.len() {
        if count[i] == max_count {
            res.push(&text[i..i+k]);
        }
    }
    // reduplicate and return
    res.sort();
    res.dedup();
    res
}

/// find most frequent k-mers with mismatches in a string.
pub fn frequent_words_with_mismatches(text: &str, k: usize, d: usize) -> Vec<String> {
    let mut res = HashSet::new();
    let len = 4usize.pow(k as u32);
    let mut close = vec![0; len];

    for i in 0..text.len()-k+1 {
        let neighborhood = self::neighbors(&text[i..i+k], d);
        for pattern in neighborhood {
            let index = pattern_to_number(pattern.as_bytes());
            close[index] += 1;
        }
    }

    let max_count = *close.iter().max().unwrap();
    for i in 0..len-1 {
        if close[i] == max_count {
            let pattern = number_to_pattern(i, k);
            res.insert(pattern);
        }
    }

    res.into_iter().collect()
}

pub fn frequent_words_with_mismatches_and_reverse_complements(text: &str, k: usize, d: usize) -> Vec<String> {
    let mut res = HashSet::new();
    let len = 4usize.pow(k as u32);
    let mut close = vec![0; len];

    for i in 0..text.len()-k+1 {
        let kmer = &text[i..i+k];
        let rkmer = reverse_complement(kmer);
        let n1 = self::neighbors(kmer, d);
        let n2 = self::neighbors(&rkmer, d);
        for pattern in n1.iter().chain(n2.iter()) {
            let index = pattern_to_number(pattern.as_bytes());
            close[index] += 1;
        }
    }

    let max_count = *close.iter().max().unwrap();
    for i in 0..len-1 {
        if close[i] == max_count {
            let pattern = number_to_pattern(i, k);
            res.insert(pattern);
        }
    }

    res.into_iter().collect()
}

/// all distinct k-mers in lexicographical order
pub fn kmers(text: &str, k: usize) -> Vec<&str> {
    let mut res = Vec::new();
    for i in 0..text.len()-k+1 {
        res.push(&text[i..i+k]);
    }
    res.sort();
    res.dedup();
    res
}

fn symbol_to_number(sym: u8) -> usize {
    match sym as char {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        'T' => 3,
        c => panic!("symbol_to_number: invalid char {}", c)
    }
}

fn number_to_symbol(n: usize) -> char {
    match n {
        0 => 'A',
        1 => 'C',
        2 => 'G',
        3 => 'T',
        x => panic!("number_to_symbol: invalid number {}", x)
    }
}

pub fn pattern_to_number(pat: &[u8]) -> usize {
    if pat.len() == 0 {
        0
    } else {
        let (init, last) = pat.split_at(pat.len()-1);
        4 * pattern_to_number(init) + symbol_to_number(last[0])
    }
}

pub fn number_to_pattern(index: usize, k: usize) -> String {
    if k == 1 {
        let mut s = String::new();
        s.push(number_to_symbol(index));
        s
    } else {
        let (quot, rem) = (index / 4, index % 4);
        let symbol = number_to_symbol(rem);
        let mut pat = number_to_pattern(quot, k-1);
        pat.push(symbol);
        pat
    }
}

pub fn frequency_array(text: &str, k: usize) -> Vec<usize> {
    let len = 4usize.pow(k as u32);
    let mut freqs = vec![0; len];
    for i in 0..text.len()-k+1 {
        let j = pattern_to_number(&text[i..i+k].as_bytes());
        freqs[j] += 1;
    }
    freqs
}

pub fn clump_finding_naive(text: &str, k: usize, l: usize, t: usize) -> Vec<&str> {
    let len = text.len() - l + 1;
    let mut res = Vec::new();
    for i in 0..len {
        let chunk = &text[i..i+l];
        let count = self::frequent_words_count(chunk, k);

        for j in 0..count.len() {
            if count[j] == t {
                res.push(&chunk[j..j+k]);
            }
        }
    }
    res.sort();
    res.dedup();
    res
}

pub fn clump_finding(genome: &str, k: usize, l: usize, t: usize) -> Vec<String> {
    let mut res = Vec::new();
    let len = 4usize.pow(k as u32);
    let mut clump = vec![0; len];
    let text = &genome[0..l];
    let mut freqs = frequency_array(text, k);

    for i in 0..len-1 {
        if freqs[i] >= t {
            clump[i] = 1;
        }
    }

    for i in 1..genome.len()-l+1 {
        let first_pat = &genome[i-1..i-1+k];
        let index = pattern_to_number(first_pat.as_bytes());
        freqs[index] -= 1;

        let last_pat = &genome[i+l-k..i+l];
        let index = pattern_to_number(last_pat.as_bytes());
        freqs[index] += 1;

        if freqs[index] >= t {
            clump[index] = 1;
        }
    }

    for i in 0..len-1 {
        if clump[i] == 1 {
            let pat = number_to_pattern(i, k);
            res.push(pat);
        }
    }

    res.sort();
    res.dedup();
    res
}

pub fn gc_skew<'a>(genome: &'a str) -> Box<Iterator<Item=isize> + 'a> {
    let iter = genome
        .chars()
        .scan(0, |acc, c| {
            *acc = match c {
                'G' => *acc + 1,
                'C' => *acc - 1,
                _ => *acc
            };
            Some(*acc)
        });
    Box::new(iter)
}

pub fn gc_skew_scan<'a>(genome: &'a str) -> Scan<Chars<'a>, isize, fn(&mut isize, char) -> Option<isize>> {

    fn skew_fn(acc: &mut isize, c: char) -> Option<isize> {
        *acc = match c {
            'G' => *acc + 1,
            'C' => *acc - 1,
            _ => *acc
        };
        Some(*acc)
    }

    genome
        .chars()
        .scan(0, skew_fn)

}

pub fn min_indices<I: Iterator<Item=isize>>(iter: I) -> (isize, Vec<usize>) {
    let mut inds = Vec::new();
    let mut min = isize::max_value();

    for (i, x) in iter.enumerate() {
        if x < min {
            inds = vec![i];
            min = x;
        } else if x == min {
            inds.push(i);
        }
    }

    (min, inds)
}

pub fn hamming_distance(xs: &str, ys: &str) -> usize {
    let (xi, yi) = (xs.chars(), ys.chars());
    xi.zip(yi).fold(0, |acc, (x, y)| {
        if x == y { acc } else { acc + 1 }
    })
}

pub fn neighbors(pattern: &str, d: usize) -> Vec<String> {
    let mut res = HashSet::new();
    if d == 0 {
        res.insert(pattern.to_string());
        res.into_iter().collect()
    } else if pattern.len() == 1 {
        res.insert(A.to_owned());
        res.insert(T.to_owned());
        res.insert(G.to_owned());
        res.insert(C.to_owned());
        res.into_iter().collect()
    } else {
        let tail = &pattern[1..];
        let suffix = neighbors(tail, d);
        for text in suffix.iter() {
            if hamming_distance(tail, text) < d {
                res.insert(A.to_owned() + text);
                res.insert(T.to_owned() + text);
                res.insert(G.to_owned() + text);
                res.insert(C.to_owned() + text);
            } else {
                let h = pattern[0..1].to_owned();
                res.insert(h + text);
            }
        }
        res.into_iter().collect()
    }
}

fn reverse_complement(text: &str) -> String {
    super::u8::Dna::from_str_unchecked(text).reverse_complement().to_string()
}

#[cfg(test)]
mod tests {

    use test::Bencher;

    use data::Dataset;

    #[test]
    fn test_indexes() {
        let answer = Dataset::open_text("data/bioinformatics1/pattern_count/dataset_2_7.out");
        let dataset = Dataset::open_text("data/bioinformatics1/pattern_count/dataset_2_7.txt");
        let lines = dataset.lines();
        let (text, pattern) = (lines[0], lines[1]);
        assert_eq!(super::indexes(text, pattern).len(), answer.parse::<usize>().unwrap());
    }

    #[bench]
    fn bench_indexes(b: &mut Bencher) {
        let dataset = Dataset::open_text("data/bioinformatics1/pattern_count/dataset_2_7.txt");
        let lines = dataset.lines();
        let (text, pattern) = (lines[0], lines[1]);
        b.iter(|| super::indexes(text, pattern));
    }
}
