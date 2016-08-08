pub mod dna;
pub mod io;
pub mod strings;

use std::cmp::PartialEq;

use dna::u8::DNA;

/// find occurrences of pattern in dna
/// return vector of starting positions of pattern in dna
pub fn find(dna: DNA, pat: DNA) -> Vec<usize> {
    let mut res = Vec::new();
    for (i, w) in dna.seq.windows(pat.len()).enumerate() {
        if w == pat.seq.as_slice() {
            res.push(i);
        }
    }
    res
}

pub fn hamming_distance<T: PartialEq>(xs: &[T], ys: &[T]) -> usize {
    xs.iter().zip(ys.iter())
        .fold(0, |acc, (x, y)| if x == y { acc } else { acc + 1 })
}

#[cfg(test)]
mod tests {

    #[test]
    fn hamming_distance() {
        let xs = "GGGCCGTTGGT";
        let ys = "GGACCGTTGAC";
        assert_eq!(super::hamming_distance(xs.as_bytes(), ys.as_bytes()), 3);
    }
}
