//! Generic algorithms on sequences.

#![feature(test)]

extern crate rand;
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

/// Returns all permutations of length `n` for elements in `xs`.
pub fn permutations_with_repetitions<'a, T>(xs: &'a [T], n: usize) -> Box<Iterator<Item=Vec<T>> + 'a>
    where T: Clone + 'a {
    match n {
        0 => Box::new(Vec::new().into_iter()),
        1 => {
            let it = xs.into_iter().cloned()
                .map(|t| vec![t]);
            Box::new(it)
        },
        _ => {
            let it = xs.into_iter().cloned()
                .flat_map(move |t| {
                    permutations_with_repetitions(xs.clone(), n - 1)
                        .map(move |perm| prepend(t.clone(), &perm))
                });
            Box::new(it)
        },
    }
}

/// Append element to the end of a slice.
///
/// # Examples
///
/// ```
/// use bio::append;
///
/// assert_eq!(append(&[1, 2, 3], 0), [1, 2, 3, 0]);
/// ```
#[inline]
pub fn append<T: Clone>(xs: &[T], x: T) -> Vec<T> {
    let mut vec = Vec::with_capacity(xs.len() + 1);
    vec.extend(xs.iter().cloned());
    vec.push(x);
    vec
}

/// Prepend element to the begining of a slice.
///
/// # Example
///
/// ```
/// use bio::prepend;
///
/// assert_eq!(prepend(0, &[1, 2, 3]), [0, 1, 2, 3]);
/// ```
#[inline]
pub fn prepend<T: Clone>(x: T, xs: &[T]) -> Vec<T> {
    let mut vec = Vec::with_capacity(xs.len() + 1);
    vec.push(x);
    vec.extend(xs.iter().cloned());
    vec
}

/// Returns concatenation of two slices.
///
/// # Example
///
/// ```
/// use bio::add;
///
/// let xs = &[1, 2];
/// let ys = &[3, 4];
/// assert_eq!(vec![1, 2, 3, 4], add(xs, ys));
/// ```
#[inline]
pub fn add<T: Clone>(xs: &[T], ys: &[T]) -> Vec<T> {
    let mut v = Vec::with_capacity(xs.len() + ys.len());
    v.extend(xs.iter().cloned());
    v.extend(ys.iter().cloned());
    v
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

    #[test]
    fn permutations_with_repetitions() {
        let input = vec![0, 1];
        let output = [vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]];
        let perms: Vec<_> = super::permutations_with_repetitions(&input, 2).collect();
        assert_eq!(perms, output);

    }

    #[bench]
    fn bench_hamming_distance(b: &mut Bencher) {
        let dataset = Dataset::open_text("data/hamming_distance/dataset_9_3.txt");
        let lines = dataset.lines();
        let (da, db) = (test::black_box(lines[0].as_bytes()), test::black_box(lines[1].as_bytes()));
        b.iter(|| super::hamming_distance(da, db))
    }

}
