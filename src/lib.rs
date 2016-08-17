#![feature(test)]

extern crate test;

pub mod data;
pub mod dna;
pub mod io;
pub mod strings;

use std::cmp::PartialEq;

use dna::u8::Dna;

/// find occurrences of pattern in dna
/// return vector of starting positions of pattern in dna
pub fn find(dna: Dna, pat: Dna) -> Vec<usize> {
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

    use test;
    use test::Bencher;

    use data::Dataset;

    static DX: &'static str = "GGGCCGTTGGT";
    static DY: &'static str = "GGACCGTTGAC";

    #[test]
    fn test_hamming_distance() {
        assert_eq!(super::hamming_distance(DX.as_bytes(), DY.as_bytes()), 3);
    }


    #[bench]
    fn bench_hamming_distance(b: &mut Bencher) {
        let (bx, by) = (test::black_box(DX.as_bytes()), test::black_box(DY.as_bytes()));
        b.iter(|| super::hamming_distance(bx, by));
    }

    #[bench]
    fn bench_hamming_distance_dataset(b: &mut Bencher) {
        let dataset = Dataset::open_text("data/hamming_distance/dataset_9_3.txt");
        let lines = dataset.lines();
        let (da, db) = (test::black_box(lines[0].as_bytes()), test::black_box(lines[1].as_bytes()));
        b.iter(|| super::hamming_distance(da, db))
    }

}
