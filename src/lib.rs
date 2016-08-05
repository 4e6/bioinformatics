pub mod dna;
pub mod strings;

use dna::u8::DNA;

pub fn find(dna: DNA, ori: DNA) -> Vec<usize> {
    let mut res = Vec::new();
    for (i, w) in dna.seq.windows(ori.len()).enumerate() {
        if w == ori.seq.as_slice() {
            res.push(i);
        }
    }
    res
}
