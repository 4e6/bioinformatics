//! Algorightms on sequences

use std::iter;

/// Search for occurrences of `pattern` in `text`. Returns indices
/// of the first character of all `text` slices that matches the `pattern`.
pub fn find<T: PartialEq>(text: &[T], pattern: &[T]) -> Vec<usize> {
    let (inds, _) = find_by(text, pattern, |a, b| a == b);
    inds
}

/// Fuzzy search of a `pattern` in `text` by a given `compare` function.
/// Returns a pair containing vector of indices and the vector of matched slices.
pub fn find_by<'a, 'b, F, T>(text: &'a [T], pattern: &'b [T], compare: F) -> (Vec<usize>, Vec<&'a [T]>)
    where F: Fn(&[T], &[T]) -> bool,
{
    text.windows(pattern.len())
        .enumerate()
        .filter(|&(_, chunk)| compare(chunk, pattern))
        .unzip()
}


/// Compute Hamming distance between two slices.
///
/// # Examples
///
/// ```
/// use bio::seq::hamming_distance;
///
/// let dx = "GGGCCGTTGGT";
/// let dy = "GGACCGTTGAC";
/// assert_eq!(hamming_distance(dx.as_bytes(), dy.as_bytes()), 3);
/// ```
///
/// # Panics
///
/// Panics when input sequences has different length.
pub fn hamming_distance<I, J, T>(xs: I, ys: J) -> usize
    where I: IntoIterator<Item = T>,
          J: IntoIterator<Item = T>,
          T: PartialEq,
{
    xs.into_iter()
        .zip(ys.into_iter())
        .fold(0, |acc, (x, y)| if x == y { acc } else { acc + 1 })
}

/// Returns indices of minimum elements.
///
/// # Examples
///
/// ```
/// use bio::seq::min_indices;
///
/// let empty: Vec<u8> = Vec::new();
///
/// assert_eq!(min_indices(vec![1, 3, 1, 2]), [0, 2]);
/// assert_eq!(min_indices(empty), []);
/// ```
pub fn min_indices<I, T>(iter: I) -> Vec<usize>
    where I: IntoIterator<Item = T>,
          T: PartialOrd + PartialEq,
{
    let mut enumerated = iter.into_iter().enumerate();
    match enumerated.next() {
        Some((i, x)) => min_inds(enumerated, vec![i], x),
        None => Vec::new(),
    }
}

fn min_inds<I, T>(enumerated: I, min_indices: Vec<usize>, local_min: T) -> Vec<usize>
    where I: Iterator<Item = (usize, T)>,
          T: PartialOrd + PartialEq,
{
    let mut min = local_min;
    let mut inds = min_indices;

    for (i, x) in enumerated {
        if x < min {
            inds.clear();
            inds.push(i);
            min = x;
        } else if x == min {
            inds.push(i);
        }
    }

    inds
}

/// Returns all permutations of length `n` for elements in `xs`.
///
/// # Examples
///
/// ```
/// use bio::seq::permutations_with_repetitions;
///
/// let input = [0, 1];
/// let expected = [vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]];
/// let res: Vec<_> = permutations_with_repetitions(&input, 2).collect();
/// assert_eq!(res, expected);
/// ```
pub fn permutations_with_repetitions<'a, T>(xs: &'a [T], n: usize) -> Box<Iterator<Item = Vec<T>> + 'a>
    where T: Clone,
{
    match n {
        0 => Box::new(iter::empty()),
        1 => {
            let it = xs.into_iter()
                .map(|t| vec![t.clone()]);
            Box::new(it)
        },
        _ => {
            let it = xs.into_iter()
                .flat_map(move |t| {
                    permutations_with_repetitions(xs.clone(), n - 1)
                        .map(move |perm| [vec![t.clone()], perm].concat())
                });
            Box::new(it)
        },
    }
}

/// Returns concatenation of two slices.
///
/// # Example
///
/// ```
/// use bio::seq::concat;
///
/// let xs = [1, 2];
/// let ys = [3, 4];
/// assert_eq!(vec![1, 2, 3, 4], concat(&xs, &ys));
/// ```
#[inline]
pub fn concat<T: Clone>(xs: &[T], ys: &[T]) -> Vec<T> {
    let mut v = Vec::with_capacity(xs.len() + ys.len());
    v.extend(xs.iter().cloned());
    v.extend(ys.iter().cloned());
    v
}


#[cfg(test)]
mod tests {

    use super::*;

    use test::Bencher;
    use data::Dataset;

    #[bench]
    fn bench_hamming_distance(b: &mut Bencher) {
        let dataset = Dataset::open_text("data/bioinformatics1/hamming_distance/dataset_9_3.txt");
        let lines = dataset.lines();
        let (da, db) = (lines[0].as_bytes(), lines[1].as_bytes());
        b.iter(|| hamming_distance(da, db));
    }

    #[bench]
    fn bench_find(b: &mut Bencher) {
        let dataset = Dataset::open_text("data/bioinformatics1/pattern_count/dataset_2_7.txt");
        let lines = dataset.lines();
        let (text, pattern) = (lines[0].as_bytes(), lines[1].as_bytes());
        b.iter(|| find(text, pattern));
    }
}
