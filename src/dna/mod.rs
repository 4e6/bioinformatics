//! Algorithms on DNA sequences

mod dna_impl;

use std::collections::HashSet;
use std::f64;

use rand;
use rand::distributions::{IndependentSample, Range, WeightedChoice, Weighted};

pub use self::dna_impl::Dna;
use ::seq;

type Seq = Vec<u8>;

pub const A: u8 = b'A';
pub const T: u8 = b'T';
pub const G: u8 = b'G';
pub const C: u8 = b'C';

pub static ALPHABET: [u8; 4] = [A, T, G, C];

/// Returns all permutations of `text` within Hamming distance of `d`.
pub fn neighbors(text: &[u8], d: usize) -> Vec<Vec<u8>> {
    let mut res = HashSet::new();
    if d == 0 {
        res.insert(text.to_vec());
        res.into_iter().collect()
    } else if text.len() == 1 {
        for a in ALPHABET.iter().cloned() {
            res.insert(vec![a]);
        }
        res.into_iter().collect()
    } else {
        let tail = &text[1..];
        let suffixes = neighbors(tail, d);
        for suffix in suffixes.iter() {
            if seq::hamming_distance(tail, suffix) < d {
                for a in ALPHABET.iter().cloned() {
                    res.insert(seq::concat(&[a], suffix));
                }
            } else {
                let h = &text[0..1];
                res.insert(seq::concat(h, suffix));
            }
        }
        res.into_iter().collect()
    }
}

/// Returns distance between `pattern` and DNA strings `dnas`
fn distance(dnas: &[Dna], pattern: &Dna) -> usize {
    let k = pattern.len();
    let mut distance = 0;

    for dna in dnas.iter() {
        let mut h = usize::max_value();
        for kmer in dna.windows(k) {
            let d = seq::hamming_distance(pattern, kmer);
            if h > d {
                h = d;
            }
        }
        distance += h;
    }

    distance
}

////////////////////////////////////////////////////////////
// Functions to work with Motifs
////////////////////////////////////////////////////////////

/// Given a collection of strings Dna and an integer d, a k-mer is a
/// (k,d)-motif if it appears in every string from Dna with at most d
/// mismatches.
pub fn motif_enumeration(dnas: &[Dna], k: usize, d: usize) -> HashSet<Dna> {
    let mut motifs = HashSet::new();
    let ref dna0 = dnas[0];
    for kmer in dna0.windows(k) {
        for kdmer in neighbors(kmer, d).iter() {
            let all_contains = dnas.iter().all(|dna| {
                let (inds, _) = seq::find_by(dna, kdmer, |a, b| seq::hamming_distance(a, b) <= d);
                inds.len() > 0
            });
            if all_contains {
                motifs.insert(Dna::from_slice(kdmer));
            }
        }
    }
    motifs
}

/// Returns median of length `k` for the vector of DNA strings `dnas`.
pub fn median_string(dnas: &[Dna], k: usize) -> Dna {
    let mut d = usize::max_value();
    let mut median = Dna::new(vec![]);
    for kmer in seq::permutations_with_repetitions(&ALPHABET, k) {
        let pattern = Dna::new(kmer);
        let dk_distance = distance(dnas, &pattern);
        if d > dk_distance {
            d = dk_distance;
            median = pattern;
        }
    }
    median
}

/// DNA Profile for motif matrix for which `P_i,j` is the frequency of
/// the i-th nucleotide in the j-th column of the motif matrix.
///
/// Note that the elements of any column of the profile matrix sum to 1.
pub struct Profile {
    a: Vec<f64>,
    c: Vec<f64>,
    g: Vec<f64>,
    t: Vec<f64>,
}

impl Profile {

    /// Creates new `Profile` matrix from given rows.
    pub fn new(pa: Vec<f64>, pc: Vec<f64>, pg: Vec<f64>, pt: Vec<f64>) -> Profile {
        assert_eq!(pa.len(), pc.len());
        assert_eq!(pa.len(), pg.len());
        assert_eq!(pa.len(), pt.len());
        Profile { a: pa, c: pc, g: pg, t: pt }
    }

    /// Return profile vector for nucleotide `n`. Where `profile[i]` is a frequency
    /// of nucleotide `n` in the `i`-th column of matrix `motifs`.
    fn vector<F>(n: u8, motifs: &[Dna], avg: &F) -> Vec<f64>
        where F: Fn(&mut f64, f64) {

        let len = motifs.len() as f64;
        let mut profile = vec![0.; motifs[0].len()];

        for motif in motifs.iter() {
            for (p, m) in profile.iter_mut().zip(motif.iter()) {
                if *m == n {
                    *p += 1.;
                }
            }
        }

        for p in profile.iter_mut() {
            avg(p, len)
        }

        profile
    }

    /// Build a `Profile` for `motifs` matrix using default `avg_mean`
    /// average algorithm.
    pub fn from_motifs(motifs: &[Dna]) -> Profile {
        Profile::build(motifs, &Profile::avg_mean)
    }

    /// Build a `Profile` for `motifs` matrix using `avg` average
    /// algorithm.
    pub fn build<F>(dnas: &[Dna], avg: &F) -> Profile
        where F: Fn(&mut f64, f64) {

        let pa = Profile::vector(A, &dnas, avg);
        let pc = Profile::vector(C, &dnas, avg);
        let pg = Profile::vector(G, &dnas, avg);
        let pt = Profile::vector(T, &dnas, avg);

        Profile::new(pa, pc, pg, pt)
    }

    /// Length (width) of `Prifile` matrix
    fn len(&self) -> usize {
        self.a.len()
    }

    /// Profile value for `nuc` at position `i`.
    fn value(&self, nuc: u8, i: usize) -> f64 {
        match nuc {
            A => self.a[i],
            C => self.c[i],
            G => self.g[i],
            T => self.t[i],
            _ => panic!("Unsupported character {}", nuc as char),
        }
    }

    /// `i`-th column with labels
    fn column(&self, i: usize) -> [(u8, f64); 4] {
        [(A, self.a[i]), (C, self.c[i]), (G, self.g[i]), (T, self.t[i])]
    }

    /// Most popular nucleotide in `i`-th column.
    fn most_popular(&self, i: usize) -> u8 {
        let mut col = self.column(i);
        col.sort_by(|&(_, fa), &(_, fb)| fb.partial_cmp(&fa).unwrap());
        let (nuc, _) = col[0];
        nuc
    }

    /// Updates `p` to an average of `l`. Used as an update function for
    /// `vector`.
    #[inline]
    pub fn avg_mean(p: &mut f64, l: f64) { *p = *p / l }

    /// Updates `p` to a normalized average using Laplace's Rule of
    /// Succession algorithm. Used as an update funiction for
    /// `vector`
    #[inline]
    pub fn avg_laplace(p: &mut f64, l: f64) { *p = (*p + 1.) / (2. * l) }

}

/// Compute probabilities of k-mers based on given probability distribution.
pub fn kmer_probabilities<'a>(dna: &'a Dna, k: usize, p: &'a Profile) -> Box<Iterator<Item = (f64, Dna)> + 'a> {
    let it = dna.windows(k)
        .map(move |kmer| (probability(kmer, p), Dna::from_slice(kmer)));
    Box::new(it)
}

/// Search for a kmer with highest probability given `p` Profile.
pub fn most_probable_kmer(dna: &Dna, k: usize, p: &Profile) -> (f64, Dna) {
    kmer_probabilities(dna, k, p)
        .fold((f64::MIN, Dna::new(vec![])), |(acc, d), (score, dna)| if score > acc { (score, dna) } else { (acc, d) } )
}

/// Greedy algorithm for motif finding.
pub fn greedy_motif_search(dnas: &[Dna], k: usize, with_pseudocounts: bool) -> Vec<Dna> {
    let update = if with_pseudocounts { Profile::avg_laplace } else { Profile::avg_mean };
    let mut best_motifs: Vec<_> = dnas.iter()
        .map(|dna| Dna::from_slice(&dna[0..k]))
        .collect();

    for kmer in dnas[0].windows(k) {
        let mut motifs = Vec::with_capacity(dnas.len());
        motifs.push(Dna::from_slice(kmer));
        for dna in dnas[1..].iter() {
            let p = Profile::build(&motifs, &update);
            let (_, most_probable) = most_probable_kmer(&dna, k, &p);
            motifs.push(most_probable);
        }
        if score(&motifs) < score(&best_motifs) {
            best_motifs = motifs;
        }
    }

    best_motifs
}

/// Single iteration of randomized algorithm for motif finding.
fn randomized_motif_search_iteration(dnas: &[Dna], k: usize) -> Vec<Dna> {
    let mut rng = rand::thread_rng();
    let range = Range::new(0, dnas[0].len() - k);
    let mut motifs: Vec<_> = dnas.iter()
        .map(|dna| Dna::from_slice(dna.kmer(k, range.ind_sample(&mut rng))))
        .collect();
    let mut best_motifs = motifs.clone();

    loop {
        let p = Profile::build(&motifs, &Profile::avg_laplace);
        let ms: Vec<_> = dnas.iter()
            .fold(Vec::with_capacity(dnas.len()), |mut acc, dna| {
                let (_, most_probable) = most_probable_kmer(&dna, k, &p);
                acc.push(most_probable);
                acc
            });
        motifs = ms;
        if score(&motifs) < score(&best_motifs) {
            best_motifs = motifs.clone();
        } else {
            break;
        }
    }

    best_motifs
}

/// Randomized alogirithm for motif finding.
pub fn randomized_motif_search(dnas: &[Dna], k: usize, iters: usize) -> Vec<Dna> {
    assert!(iters > 0);
    let mut best_motifs: Vec<_> = dnas.iter()
        .map(|dna| Dna::from_slice(&dna[0..k]))
        .collect();

    for _ in 0..iters {
        let motifs = randomized_motif_search_iteration(dnas, k);
        if score(&motifs) < score(&best_motifs) {
            best_motifs = motifs;
        }
    }

    best_motifs
}


/// Single Gibbs sampler algorithm iteration
pub fn gibbs_sampler_iteration(dnas: &[Dna], k: usize, t: usize, n: usize) -> Vec<Dna> {
    let mut rng = rand::thread_rng();
    let kmer_range = Range::new(0, dnas[0].len() - k);
    let motif_range = Range::new(0, t);
    let mut motifs: Vec<_> = dnas.iter()
        .map(|dna| Dna::from_slice(dna.kmer(k, kmer_range.ind_sample(&mut rng))))
        .collect();
    let mut best_motifs = motifs.clone();

    for _ in 0..n {
        let i = motif_range.ind_sample(&mut rng);
        motifs.remove(i);
        let p = Profile::build(&motifs, &Profile::avg_laplace);
        let motif = randomly_generated(&dnas[i], k, &p, &mut rng);
        motifs.insert(i, motif);

        if score(&motifs) < score(&best_motifs) {
            best_motifs = motifs.clone();
        }
    }

    best_motifs
}

/// Randomized Gibbs sampler algorithm for motif finding.
pub fn gibbs_sampler(dnas: &[Dna], k: usize, t: usize, n: usize, iters: usize) -> Vec<Dna> {
    assert!(iters > 0);
    let mut best_motifs: Vec<_> = dnas.iter()
        .map(|dna| Dna::from_slice(&dna[0..k]))
        .collect();

    for _ in 0..iters {
        let motifs = gibbs_sampler_iteration(dnas, k, t, n);
        if score(&motifs) < score(&best_motifs) {
            best_motifs = motifs;
        }
    }

    best_motifs
}

/// Return score, as a cumulative Hamming distance between `consensus`
/// string for `motifs` matrix and `motifs` matrix itself.
fn score(motifs: &[Dna]) -> usize {
    distance(motifs, &consensus(motifs))
}

/// Return probability of occurrence of the `dna` sequence given `p`
/// probability distribution.
fn probability(dna: &[u8], p: &Profile) -> f64 {
    dna.iter()
        .enumerate()
        .fold(1., |acc, (i, &c)| acc * p.value(c, i))
}

/// Returns weighted random kmer of `dna` based on `p` probability distribution.
fn randomly_generated<R: rand::Rng>(dna: &[u8], k: usize, p: &Profile, rng: &mut R) -> Dna {
    let ceil = u16::max_value() as u32;
    // bigger probability distribution gives better result
    //let ceil = u32::max_value() / 100;
    let probabilities: Vec<_> = dna.windows(k)
        .map(|kmer| (probability(kmer, &p), kmer))
        .collect();
    let (min, max) = probabilities.iter()
        .fold((f64::MAX, f64::MIN), |acc, &(pr, _)| (acc.0.min(pr), acc.1.max(pr)));
    // maps f64 probabilities to the u32 in a range 1...ceil
    let (m, c) = scale_coeffs(min, max, 0, ceil);
    let mut weights: Vec<_> = probabilities.iter()
        .map(|&(pr, kmer)| Weighted {
            weight: pr.mul_add(m, c).ceil() as u32,
            item: kmer
        }).collect();

    let wc = WeightedChoice::new(&mut weights);
    Dna::from_slice(wc.ind_sample(rng))
}

/// Return coefficients to scale interval [min, max] into interval [a, b].
///
/// Solves following system of linear equations:
///
/// ```text
/// b = m*max + c
/// a = m*min + c
/// ```
///
/// Returns a pair of coefficients (m, c) that should be used to scale x:
///
/// ```text
/// scaled = x*m + c
/// ```
///
/// # Example
///
/// Scale value from range [0, 1] to [0, 10]:
///
/// ```
/// use bio::dna::scale_coeffs;
///
/// let (m, c) = scale_coeffs(0., 1., 0, 10);
/// let x = 0.5_f64;
/// assert_eq!(x.mul_add(m, c), 5.0);
/// ```
pub fn scale_coeffs(min: f64, max: f64, a: u32, b: u32) -> (f64, f64) {
    let c = (a as f64 * max - b as f64 * min) / (max - min);
    let m = (b - a) as f64 / (max - min);
    (m, c)
}

/// Return consensus DNA string from the most popular letters in each column of
/// the `motifs` matrix.
fn consensus(motifs: &[Dna]) -> Dna {
    let mut vec = Vec::new();
    let p = Profile::from_motifs(motifs);

    for i in 0..p.len() {
        vec.push(p.most_popular(i));
    }

    Dna::new(vec)
}

#[cfg(test)]
mod tests {

    use super::Dna;

    #[test]
    fn distance() {
        let pattern = Dna::from_str("AAA");
        let dnas: Vec<_> = ["TTACCTTAAC", "GATATCTGTC", "ACGGCGTTCG", "CCCTAAAGAG", "CGTCAGAGGT"]
            .iter()
            .map(|x| Dna::from_str(x))
            .collect();
        assert_eq!(super::distance(&dnas, &pattern), 5);
    }

}
