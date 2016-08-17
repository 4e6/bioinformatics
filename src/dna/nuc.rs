use std::mem;
use std::str;

/// Nucleic Acid Code
#[repr(u8)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub enum Nuc {
    A = 'A' as u8,
    C = 'C' as u8,
    G = 'G' as u8,
    T = 'T' as u8,
}


impl Nuc {

    pub fn to_utf8(nuc: Nuc) -> u8 {
        nuc as u8
    }

    pub fn from_utf8(x: u8) -> Nuc {
        use self::Nuc::*;
        match x as char {
            'A' => A,
            'C' => C,
            'G' => G,
            'T' => T,
            c => panic!("Unsupported Nuc: '{}'", c),
        }
    }

    pub unsafe fn from_utf8_unchecked(x: u8) -> Nuc {
        mem::transmute(x)
    }

    pub fn complement(nuc: Nuc) -> Nuc {
        use self::Nuc::*;
        match nuc {
            A => T,
            T => A,
            G => C,
            C => G,
        }
    }

}


#[derive(PartialOrd, PartialEq, Eq, Ord)]
pub struct Dna {
    vec: Vec<Nuc>,
}

impl Dna {

    pub fn from_slice(s: &[u8]) -> Dna {
        let vec = s.iter().cloned()
            .map(Nuc::from_utf8)
            .collect();
        Dna { vec: vec }
    }

    pub unsafe fn from_slice_unchecked(s: &[u8]) -> Dna {
        let vec: &[Nuc] = mem::transmute(s);
        Dna { vec: vec.to_vec() }
    }

    pub fn from_str(s: &str) -> Dna {
        Dna::from_slice(s.as_bytes())
    }

    pub unsafe fn from_str_unchecked(s: &str) -> Dna {
        Dna::from_slice_unchecked(s.as_bytes())
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn into_vec(self) -> Vec<Nuc> {
        self.vec
    }

    pub fn as_slice(&self) -> &[Nuc] {
        self.vec.as_slice()
    }

    pub fn to_utf8(&self) -> Vec<u8> {
        self.vec.iter().cloned()
            .map(Nuc::to_utf8)
            .collect()
    }

    pub unsafe fn to_utf8_unchecked(&self) -> &[u8] {
        mem::transmute(self.vec.as_slice())
    }

    pub fn to_string(&self) -> String {
        unsafe { String::from_utf8_unchecked(self.to_utf8()) }
    }

    pub unsafe fn into_string_unchecked(self) -> String {
        String::from_utf8_unchecked(mem::transmute(self.vec))
    }

    // pub fn as_str(&self) -> &str {
    //     //let v = self.to_utf8();
    //     str::from_utf8(self.vec.as_slice()).unwrap()
    // }

    pub fn complement(&self) -> Dna {
        let cmp = self.vec.iter().cloned()
            .map(Nuc::complement)
            .collect();
        Dna { vec: cmp }
    }

    pub fn reverse(&mut self) {
        self.vec.reverse();
    }

    pub fn reverse_complement(&self) -> Dna {
        let mut comp = self.complement();
        comp.reverse();
        comp
    }

    pub fn find<F>(&self, pattern: &Dna, p: F) -> (Vec<usize>, Vec<&[Nuc]>)
        where F: Fn(&[Nuc], &[Nuc]) -> bool {

        self.vec
            .windows(pattern.len())
            .enumerate()
            .filter(|&(_, w)| p(w, pattern.as_slice()))
            .unzip()
    }
}

#[cfg(test)]
mod tests {

    use test::Bencher;

    use super::Dna;
    use data::Dataset;

    static SAMPLE: &'static str = "ACTATGCGACT";
    static COMPLEMENT: &'static str = "TGATACGCTGA";

    #[test]
    fn from_str() {
        let dna = Dna::from_str(SAMPLE);
        assert_eq!(SAMPLE.to_string(), dna.to_string());
    }

    #[test]
    fn from_str_unchecked() {
        let dna = unsafe { Dna::from_str_unchecked(SAMPLE) };
        assert_eq!(SAMPLE.to_string(), dna.to_string());
    }

    #[test]
    fn from_slice() {
        let bytes = SAMPLE.as_bytes();
        let dna = Dna::from_slice(bytes);
        assert_eq!(SAMPLE.to_string(), dna.to_string());
    }

    #[test]
    fn from_slice_unchecked() {
        let bytes = SAMPLE.as_bytes();
        let dna = unsafe { Dna::from_slice_unchecked(bytes) };
        assert_eq!(SAMPLE.to_string(), dna.to_string())
    }

    #[test]
    fn to_utf8() {
        let dna = Dna::from_str(SAMPLE);
        assert_eq!(SAMPLE.as_bytes().to_owned(), dna.to_utf8());
    }

    #[test]
    fn to_utf8_unchecked() {
        let dna = Dna::from_str(SAMPLE);
        let dna_bytes = unsafe { dna.to_utf8_unchecked() };
        assert_eq!(SAMPLE.as_bytes(), dna_bytes);
    }

    #[test]
    fn into_string_unchecked() {
        let dna = Dna::from_str(SAMPLE);
        assert_eq!(SAMPLE.to_owned(), unsafe { dna.into_string_unchecked() })
    }

    #[test]
    fn reverse_complement() {
        let dna = Dna::from_str(SAMPLE);
        let reverse_complement: String = COMPLEMENT.chars().rev().collect();
        assert_eq!(reverse_complement, dna.reverse_complement().to_string());
    }

    #[bench]
    fn bench_from_str(b: &mut Bencher) {
        let dataset = Dataset::open_fasta("data/Salmonella_enterica.txt");
        b.iter(|| Dna::from_str(dataset.contents()));
    }

    #[bench]
    fn bench_from_str_unchecked(b: &mut Bencher) {
        let dataset = Dataset::open_fasta("data/Salmonella_enterica.txt");
        b.iter(|| unsafe { Dna::from_str_unchecked(dataset.contents()) });
    }

    #[bench]
    fn bench_reverse_complement(b: &mut Bencher) {
        let dataset = Dataset::open_text("data/reverse_complement/dataset_3_2.txt");
        let lines = dataset.lines();
        let dna = Dna::from_str(lines[0]);
        b.iter(|| dna.reverse_complement())
    }
}
