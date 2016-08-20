//! Algorithms on `&[u8]` slices of bytes

pub mod dna;

use std::collections::HashSet;

pub use self::dna::Dna;
use ::hamming_distance;

static A: [u8; 1] = [b'A'];
static T: [u8; 1] = [b'T'];
static C: [u8; 1] = [b'C'];
static G: [u8; 1] = [b'G'];

type Text = Vec<u8>;

/// Search for occurrences of `pattern` in `text`. Returns indices
/// of the first character of all `text` slices that matches the `pattern`.
pub fn find(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    let (inds, _) = self::find_by(text, pattern, |a, b| a == b);
    inds
}

/// Fuzzy search of a `pattern` in `text` by a given `compare` function.
/// Returns a pair containing vector of indices and the vector of matched slices.
pub fn find_by<'a, 'b, F>(text: &'a [u8], pattern: &'b [u8], compare: F) -> (Vec<usize>, Vec<&'a [u8]>)
    where F: Fn(&[u8], &[u8]) -> bool {

    text.windows(pattern.len())
        .enumerate()
        .filter(|&(_, chunk)| compare(chunk, pattern))
        .unzip()
}

/// Returns all permutations of `text` within Hamming distance of `d`.
pub fn neighbors(text: &[u8], d: usize) -> Vec<Text> {
    fn add(xs: &[u8], ys: &[u8]) -> Vec<u8> {
        let mut v = Vec::with_capacity(xs.len() + ys.len());
        v.extend(xs.iter().cloned());
        v.extend(ys.iter().cloned());
        v
    }
    let mut res = HashSet::new();
    if d == 0 {
        res.insert(text.to_vec());
        res.into_iter().collect()
    } else if text.len() == 1 {
        res.insert(A.to_vec());
        res.insert(T.to_vec());
        res.insert(G.to_vec());
        res.insert(C.to_vec());
        res.into_iter().collect()
    } else {
        let tail = &text[1..];
        let suffixes = neighbors(tail, d);
        for suffix in suffixes.iter() {
            if hamming_distance(tail, suffix) < d {
                res.insert(add(&A, suffix));
                res.insert(add(&T, suffix));
                res.insert(add(&G, suffix));
                res.insert(add(&C, suffix));
            } else {
                let h = &text[0..1];
                res.insert(add(h, suffix));
            }
        }
        res.into_iter().collect()
    }
}

/// Given a collection of strings Dna and an integer d, a k-mer is a
/// (k,d)-motif if it appears in every string from Dna with at most d
/// mismatches.
pub fn motif_enumeration(dnas: &[Dna], k: usize, d: usize) -> HashSet<Dna> {
    let mut motifs = HashSet::new();
    let ref dna0 = dnas[0];
    for kmer in dna0.windows(k) {
        for kdmer in neighbors(kmer, d).iter() {
            let all_contains = dnas.iter().all(|dna| {
                let (inds, _) = self::find_by(dna, kdmer, |a, b| hamming_distance(a, b) <= d);
                inds.len() > 0
            });
            if all_contains {
                motifs.insert(Dna::from_slice(kdmer));
            }
        }
    }
    motifs
}

#[cfg(test)]
mod tests {

    use test::Bencher;

    use data::Dataset;

    #[bench]
    fn bench_find_by(b: &mut Bencher) {
        let dataset = Dataset::open_text("data/pattern_count/dataset_2_7.txt");
        let lines = dataset.lines();
        let (text, pattern) = (lines[0].as_bytes(), lines[1].as_bytes());
        b.iter(|| super::find_by(text, pattern, |a, b| a == b))
    }
}
