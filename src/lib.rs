pub mod dna;

pub use dna::DNA;

pub fn pattern_count(s: &str, k: &str) -> u32 {
    let mut count: u32 = 0;
    for i in 0..s.len()-k.len()+1 {
        if &s[i..i+k.len()] == k {
            count += 1;
        }
    }
    count
}

pub fn find(dna: DNA, ori: DNA) -> Vec<usize> {
    let mut res = Vec::new();
    for (i, w) in dna.seq.windows(ori.len()).enumerate() {
        if w == ori.seq.as_slice() {
            res.push(i);
        }
    }
    res
}
