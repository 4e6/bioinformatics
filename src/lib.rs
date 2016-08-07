pub mod dna;
pub mod io;
pub mod strings;

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
