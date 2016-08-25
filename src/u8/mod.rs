//! Algorithms on `&[u8]` slices of bytes

pub mod dna;

use std::collections::HashSet;
use std::f64;

pub use self::dna::Dna;

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
            if ::hamming_distance(tail, suffix) < d {
                res.insert(::add(&A, suffix));
                res.insert(::add(&T, suffix));
                res.insert(::add(&G, suffix));
                res.insert(::add(&C, suffix));
            } else {
                let h = &text[0..1];
                res.insert(::add(h, suffix));
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
            let d = ::hamming_distance(&pattern, kmer);
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
                let (inds, _) = self::find_by(dna, kdmer, |a, b| ::hamming_distance(a, b) <= d);
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
    for kmer in ::permutations_with_repetitions(dna::NUCS, k) {
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

        let pa = Profile::vector(dna::A, &dnas, avg);
        let pc = Profile::vector(dna::C, &dnas, avg);
        let pg = Profile::vector(dna::G, &dnas, avg);
        let pt = Profile::vector(dna::T, &dnas, avg);

        Profile::new(pa, pc, pg, pt)
    }

    /// Length (width) of `Prifile` matrix
    fn len(&self) -> usize {
        self.a.len()
    }

    /// Profile value for `nuc` at position `i`.
    fn value(&self, nuc: u8, i: usize) -> f64 {
        match nuc {
            dna::A => self.a[i],
            dna::C => self.c[i],
            dna::G => self.g[i],
            dna::T => self.t[i],
            _ => panic!("Unsupported character {}", nuc as char),
        }
    }

    /// `i`-th column with labels
    fn column(&self, i: usize) -> [(u8, f64); 4] {
        [(dna::A, self.a[i]), (dna::C, self.c[i]), (dna::G, self.g[i]), (dna::T, self.t[i])]
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

    use test::Bencher;

    use super::Dna;
    use data::Dataset;

    #[test]
    fn distance() {
        let pattern = Dna::from_str_unchecked("AAA");
        let dnas: Vec<_> = ["TTACCTTAAC", "GATATCTGTC", "ACGGCGTTCG", "CCCTAAAGAG", "CGTCAGAGGT"]
            .iter()
            .map(|x| Dna::from_str_unchecked(x))
            .collect();
        assert_eq!(super::distance(&dnas, &pattern), 5);
    }

    #[bench]
    fn bench_find_by(b: &mut Bencher) {
        let dataset = Dataset::open_text("data/pattern_count/dataset_2_7.txt");
        let lines = dataset.lines();
        let (text, pattern) = (lines[0].as_bytes(), lines[1].as_bytes());
        b.iter(|| super::find_by(text, pattern, |a, b| a == b))
    }
}
