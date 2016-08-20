//! Generic algorithms on sequences.

#![feature(test)]

extern crate test;

pub mod adt;
pub mod u8;
pub mod strings;

pub mod data;
pub mod io;

use std::cmp::PartialEq;

/// Compute Hamming distance between two slices.
///
/// # Panics
///
/// Panics when input sequences has different length.
pub fn hamming_distance<T: PartialEq>(xs: &[T], ys: &[T]) -> usize {
    xs.iter()
        .zip(ys.iter())
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
        let dataset = Dataset::open_text("data/hamming_distance/dataset_9_3.txt");
        let lines = dataset.lines();
        let (da, db) = (test::black_box(lines[0].as_bytes()), test::black_box(lines[1].as_bytes()));
        b.iter(|| super::hamming_distance(da, db))
    }

}
