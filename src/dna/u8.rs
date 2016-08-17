use std::str;
use std::ops::Deref;

pub const A: u8 = 'A' as u8;
pub const T: u8 = 'T' as u8;
pub const G: u8 = 'G' as u8;
pub const C: u8 = 'C' as u8;

// static NUCS: &'static [u8] = &[A, T, G, C];

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Dna {
    vec: Vec<u8>,
}

impl Dna {

    pub fn from_slice(s: &[u8]) -> Dna {
        Dna { vec: s.to_vec() }
    }

    pub fn from_str(s: &str) -> Dna {
        let bytes = s.as_bytes();
        Dna::from_slice(bytes)
    }

    pub fn to_string(&self) -> String {
        unsafe { String::from_utf8_unchecked(self.vec.clone()) }
    }

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.vec) }
    }

    pub fn as_slice(&self) -> &[u8] {
        self
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn complement(&self) -> Dna {
        let vec = self.vec.iter()
            .map(|&x| self::complement(x))
            .collect();
        Dna { vec: vec }
    }

    pub fn reverse(&mut self) {
        self.vec.reverse();
    }

    pub fn reverse_complement(&self) -> Dna {
        let mut comp = self.complement();
        comp.reverse();
        comp
    }

    pub fn find<F>(&self, pattern: &Dna, p: F) -> (Vec<usize>, Vec<&[u8]>)
        where F: Fn(&[u8], &[u8]) -> bool {
        let pat = pattern.vec.as_slice();
        self.vec
            .windows(pat.len())
            .enumerate()
            .filter(|&(_, w)| p(w, pat))
            .unzip()
    }


}

fn complement(nuc: u8) -> u8 {
    match nuc {
        A => T,
        T => A,
        G => C,
        C => G,
        x => panic!("Unsupported NUC: {}", x),
    }
}

// pub fn neighbors(pattern: &Dna, d: usize) -> Vec<Dna> {
//     let mut res = Vec::new();
//     if d == 0 {
//         res.push(pattern.clone());
//         res
//     } else if pattern.len() == 1 {
//         res.extend(NUCS.iter().cloned().map(|n| Dna { vec: vec![n] }));
//         res
//     } else {
//         let tail = &pattern.vec[1..];
//         let suffix_res = neighbors(&Dna { vec: tail.to_vec() }, d);
//         res
//     }
// }

impl Deref for Dna {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.vec
    }
}

impl Clone for Dna {
    fn clone(&self) -> Self {
        Dna { vec: self.vec.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.vec.clone_from(&source.vec);
    }
}

impl AsRef<[u8]> for Dna {
    fn as_ref(&self) -> &[u8] {
        &self
    }
}

#[cfg(test)]
mod tests {

    use super::Dna;

    static SAMPLE: &'static str = "ACTATGCGACT";

    #[test]
    fn test_from_str() {
        let dna = Dna::from_str(SAMPLE);
        assert_eq!(SAMPLE.to_string(), dna.to_string());
    }

    #[test]
    fn test_from_slice() {
        let bytes = SAMPLE.as_bytes();
        let dna = Dna::from_slice(bytes);
        assert_eq!(SAMPLE.to_string(), dna.to_string());
    }

    #[test]
    fn test_as_str() {
        let dna = Dna::from_str(SAMPLE);
        assert_eq!(SAMPLE, dna.as_str());
    }

    #[test]
    fn test_reverse_complement() {
        let dna = Dna::from_str("AAAACCCGGT");
        let reverse_complement = "ACCGGGTTTT";
        let mut comp = dna.complement();
        comp.reverse();
        assert_eq!(comp.to_string(), reverse_complement.to_string())
    }
}
