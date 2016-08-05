use std::str;

/// Nucleic Acid Code
pub enum NUC {
    A, C, G, T, U
}


impl NUC {

    // TODO: make parseable ???
    // c.parse::<NUC>()
    pub fn from_char(c: char) -> NUC {
        use self::NUC::*;
        match c {
            'A' => A,
            'C' => C,
            'G' => G,
            'T' => T,
            'U' => U,
            x => panic!("Unsupported NUC: {}", x)
        }
    }

    pub fn to_char(nuc: &NUC) -> char {
        use self::NUC::*;
        match *nuc {
            A => 'A',
            C => 'C',
            G => 'G',
            T => 'T',
            U => 'U'
        }
    }

    pub fn to_utf8(nuc: &NUC) -> u8 {
        NUC::to_char(nuc) as u8
    }

    pub fn complement(nuc: &NUC) -> NUC {
        use self::NUC::*;
        match *nuc {
            A => T,
            T => A,
            G => C,
            C => G,
            U => U
        }
    }

}
pub struct DNA {
    pub seq: Vec<NUC>
}

impl DNA {

    pub fn from_slice(s: &[u8]) -> DNA {
        let seq = s.iter()
            .map(|x| NUC::from_char(*x as char))
            .collect();
        DNA { seq: seq }
    }

    pub fn from_str(s: &str) -> DNA {
        DNA::from_slice(s.as_bytes())
    }

    pub fn len(&self) -> usize {
        self.seq.len()
    }

    // pub fn to_utf8(&self) -> Vec<u8> {
    //     self.seq.iter().clone()
    //         .map(|x| NUC::to_utf8(x))
    //         .collect()
    // }

    pub fn to_string(&self) -> String {
        let seq = self.seq.iter()
            .map(|x| NUC::to_char(x) as u8)
            .collect();
        String::from_utf8(seq).unwrap()
    }

    // pub fn as_str(&self) -> &str {
    //     let v = self.to_utf8();
    //     str::from_utf8(v.as_slice()).unwrap()
    // }

    pub fn complement(&self) -> DNA {
        let seq = self.seq.iter()
            .map(|x| NUC::complement(x))
            .collect();
        DNA { seq: seq }
    }

    pub fn reverse(&mut self) {
        self.seq.reverse();
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

}
