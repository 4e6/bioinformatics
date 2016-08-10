use std::str;

pub const A: u8 = 'A' as u8;
pub const T: u8 = 'T' as u8;
pub const G: u8 = 'G' as u8;
pub const C: u8 = 'C' as u8;

// static NUCS: &'static [u8] = &[A, T, G, C];

pub struct DNA {
    pub seq: Vec<u8>
}

impl DNA {

    pub fn len(&self) -> usize {
        self.seq.len()
    }

    pub fn from_slice(s: &[u8]) -> DNA {
        DNA { seq: s.to_vec() }
    }

    pub fn from_str(s: &str) -> DNA {
        let bytes = s.as_bytes();
        DNA::from_slice(bytes)
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.seq.clone()).unwrap()
    }

    pub fn as_str(&self) -> &str {
        str::from_utf8(self.seq.as_slice()).unwrap()
    }

    pub fn complement(&self) -> DNA {
        let seq = self.seq.iter()
            .map(|&x| self::complement(x))
            .collect();
        DNA { seq: seq }
    }

    pub fn reverse(&mut self) {
        self.seq.reverse();
    }

    pub fn reverse_complement(&self) -> DNA {
        let mut comp = self.complement();
        comp.reverse();
        comp
    }

    pub fn find<F>(&self, pattern: &DNA, p: F) -> (Vec<usize>, Vec<&[u8]>)
        where F: Fn(&[u8], &[u8]) -> bool {
        let pat = pattern.seq.as_slice();
        self.seq
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
        x => panic!("Unsupported NUC: {}", x)
    }
}

// pub fn neighbors(pattern: &DNA, d: usize) -> Vec<DNA> {
//     let mut res = Vec::new();
//     if d == 0 {
//         res.push(pattern.clone());
//         res
//     } else if pattern.len() == 1 {
//         res.extend(NUCS.iter().cloned().map(|n| DNA { seq: vec![n] }));
//         res
//     } else {
//         let tail = &pattern.seq[1..];
//         let suffix_res = neighbors(&DNA { seq: tail.to_vec() }, d);
//         res
//     }
// }

impl Clone for DNA {
    fn clone(&self) -> Self {
        DNA { seq: self.seq.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.seq.clone_from(&source.seq);
    }
}

#[cfg(test)]
mod tests {

    use super::DNA;

    static SEQ_STRING: &'static str = "ACTATGCGACT";

    #[test]
    fn test_from_str() {
        let dna = DNA::from_str(SEQ_STRING);
        assert_eq!(SEQ_STRING.to_string(), dna.to_string());
    }

    #[test]
    fn test_from_slice() {
        let bytes = SEQ_STRING.as_bytes();
        let dna = DNA::from_slice(bytes);
        assert_eq!(SEQ_STRING.to_string(), dna.to_string());
    }

    #[test]
    fn test_as_str() {
        let dna = DNA::from_str(SEQ_STRING);
        assert_eq!(SEQ_STRING, dna.as_str());
    }

    #[test]
    fn test_reverse_complement() {
        let dna = DNA::from_str("AAAACCCGGT");
        let reverse_complement = "ACCGGGTTTT";
        let mut comp = dna.complement();
        comp.reverse();
        assert_eq!(comp.to_string(), reverse_complement.to_string())
    }
}
