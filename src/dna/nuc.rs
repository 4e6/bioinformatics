use std::str;

/// Nucleic Acid Code
#[repr(u8)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub enum NUC {
    A = 'A' as u8,
    C = 'C' as u8,
    G = 'G' as u8,
    T = 'T' as u8
}


impl NUC {

    pub fn to_utf8(nuc: NUC) -> u8 {
        nuc as u8
    }

    pub fn from_utf8(x: u8) -> NUC {
        use self::NUC::*;
        match x as char {
            'A' => A,
            'C' => C,
            'G' => G,
            'T' => T,
            _ => panic!("Unsupported NUC: {}", x)
        }
    }

    pub fn complement(nuc: NUC) -> NUC {
        use self::NUC::*;
        match nuc {
            A => T,
            T => A,
            G => C,
            C => G,
        }
    }

}

#[derive(PartialOrd, PartialEq, Eq, Ord)]
pub struct DNA {
    seq: Vec<NUC>
}

impl DNA {

    pub fn from_slice(s: &[u8]) -> DNA {
        let seq = s.iter().cloned()
            .map(NUC::from_utf8)
            .collect();
        DNA { seq: seq }
    }

    pub fn from_str(s: &str) -> DNA {
        DNA::from_slice(s.as_bytes())
    }

    pub fn len(&self) -> usize {
        self.seq.len()
    }

    pub fn into_seq(self) -> Vec<NUC> {
        self.seq
    }

    pub fn as_slice(&self) -> &[NUC] {
        self.seq.as_slice()
    }

    pub fn to_utf8(&self) -> Vec<u8> {
        self.seq.iter().cloned()
            .map(NUC::to_utf8)
            .collect()
    }

    pub fn to_string(&self) -> String {
        unsafe { String::from_utf8_unchecked(self.to_utf8()) }
    }

    // pub fn as_str(&self) -> &str {
    //     //let v = self.to_utf8();
    //     str::from_utf8(self.seq.as_slice()).unwrap()
    // }

    pub fn to_complement(&self) -> DNA {
        let cmp = self.seq.iter().cloned()
            .map(NUC::complement)
            .collect();
        DNA { seq: cmp }
    }

    pub fn reverse(&mut self) {
        self.seq.reverse();
    }

    pub fn find<F>(&self, pattern: &DNA, p: F) -> (Vec<usize>, Vec<&[NUC]>)
        where F: Fn(&[NUC], &[NUC]) -> bool {

        self.seq
            .windows(pattern.len())
            .enumerate()
            .filter(|&(_, w)| p(w, pattern.as_slice()))
            .unzip()
    }
}

#[cfg(test)]
mod tests {

    use super::DNA;

    static SEQ_STRING: &'static str = "ACTATGCGACT";

    #[test]
    fn from_str() {
        let dna = DNA::from_str(SEQ_STRING);
        assert_eq!(SEQ_STRING.to_string(), dna.to_string());
    }

    #[test]
    fn from_slice() {
        let bytes = SEQ_STRING.as_bytes();
        let dna = DNA::from_slice(bytes);
        assert_eq!(SEQ_STRING.to_string(), dna.to_string());
    }

}
