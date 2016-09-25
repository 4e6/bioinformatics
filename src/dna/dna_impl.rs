//! DNA implementation based on u8 vector.

use std::ascii::AsciiExt;
use std::fmt;
use std::iter::IntoIterator;
use std::ops;
use std::slice;
use std::str;
use std::vec;

pub const A: u8 = b'A';
pub const T: u8 = b'T';
pub const G: u8 = b'G';
pub const C: u8 = b'C';

pub static ALPHABET: [u8; 4] = [A, T, G, C];

/// DNA abstraction over a byte vector.
///
/// # Examples
///
/// `Dna` implements `Deref` to a slice `&[u8]`, so it is possible to call
/// slice methods directly on `Dna`:
///
/// ```
/// use bio::dna::Dna;
///
/// let dna = Dna::from_str("AATG");
/// let mut kmers = dna.windows(2);
///
/// assert_eq!(kmers.next(), Some("AA".as_bytes()));
/// assert_eq!(kmers.next(), Some("AT".as_bytes()));
/// assert_eq!(kmers.next(), Some("TG".as_bytes()));
/// ```
#[derive(Debug, PartialEq, PartialOrd, Ord, Hash)]
pub struct Dna {
    vec: Vec<u8>,
}

impl Dna {

    pub fn new(vec: Vec<u8>) -> Dna {
        Dna { vec: vec }
    }

    pub fn from_slice(s: &[u8]) -> Dna {
        Dna { vec: s.to_vec() }
    }

    pub fn parse_from_str(s: &str) -> Result<Dna, AsciiError> {
        str::FromStr::from_str(s)
    }

    pub fn from_str(s: &str) -> Dna {
        let bytes = s.as_bytes();
        Dna::from_slice(bytes)
    }

    pub fn from_string(s: String) -> Dna {
        Dna::new(s.into_bytes())
    }

    pub fn to_string(self) -> String {
        unsafe { String::from_utf8_unchecked(self.vec) }
    }

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.vec) }
    }

    /// get kmer by index
    pub fn kmer(&self, k: usize, i: usize) -> &[u8] {
        assert!(i+k <= self.len());
        &self[i..i+k]
    }

    /// Make complement for this DNA string
    ///
    /// # Examples
    ///
    /// ```
    /// use bio::dna::Dna;
    ///
    /// let mut dna = Dna::from_str("AATG");
    /// dna.complement();
    ///
    /// assert_eq!("TTAC", dna.as_str());
    /// ```
    pub fn complement(&mut self) {
        for e in self.vec.iter_mut() {
            *e = self::complement(*e);
        }
    }

    /// Return reverse complement of the DNA string.
    ///
    /// # Examples
    ///
    /// ```
    /// use bio::dna::Dna;
    ///
    /// let dna = Dna::from_str("AATG");
    /// let rcomp = dna.reverse_complement();
    /// assert_eq!("CATT", rcomp.as_str());
    /// ```
    pub fn reverse_complement(&self) -> Dna {
        let mut dna = self.clone();
        dna.complement();
        dna.reverse();
        dna
    }

}

// Utilities

fn complement(nuc: u8) -> u8 {
    match nuc {
        A => T,
        T => A,
        G => C,
        C => G,
        x => x,
    }
}

impl Eq for Dna {}

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

impl AsMut<[u8]> for Dna {
    fn as_mut(&mut self) -> &mut [u8] {
        self.vec.as_mut()
    }
}

impl ops::Deref for Dna {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.vec
    }
}

impl ops::DerefMut for Dna {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.vec.deref_mut()
    }
}

impl fmt::Display for Dna {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl str::FromStr for Dna {
    type Err = AsciiError;

    fn from_str(s: &str) -> Result<Dna, AsciiError> {
        if s.is_ascii() {
            Ok(Dna::from_slice(s.as_bytes()))
        } else {
            let (last_valid, _) = s.char_indices()
                .take_while(|&(_, c)| c.is_ascii())
                .last()
                .unwrap();
            Err(AsciiError { valid_up_to: last_valid })
        }
    }
}

impl IntoIterator for Dna {
    type Item = u8;
    type IntoIter = vec::IntoIter<u8>;

    fn into_iter(self) -> vec::IntoIter<u8> {
        self.vec.into_iter()
    }
}

impl<'a> IntoIterator for &'a Dna {
    type Item = &'a u8;
    type IntoIter = slice::Iter<'a, u8>;

    fn into_iter(self) -> slice::Iter<'a, u8> {
        self.iter()
    }
}

/// Errors which can occur when attempting to interpret a sequence of
/// `u8` as `Dna`.
#[derive(Debug)]
pub struct AsciiError {
    valid_up_to: usize,
}

impl AsciiError {
    /// Returns the index in the given string up to which valid Ascii
    /// was verified.
    ///
    /// It is the maximum index such that `parse_from_str(input[..index])`
    /// would return Ok(_).
    ///
    /// # Examples
    ///
    /// ```
    /// use bio::dna::Dna;
    ///
    /// let s = "AAT©";
    /// let err = Dna::parse_from_str(s).unwrap_err();
    /// assert_eq!(2, err.valid_up_to());
    /// ```
    pub fn valid_up_to(&self) -> usize { self.valid_up_to }
}

impl fmt::Display for AsciiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid ascii: invalid byte near index {}", self.valid_up_to)
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
        let mut dna = Dna::from_str("AAAACCCGGT");
        let dna_rcomp = dna.reverse_complement();
        dna.complement();
        dna.reverse();
        let reverse_complement = "ACCGGGTTTT";
        assert_eq!(dna_rcomp.as_str(), reverse_complement);
        assert_eq!(dna.as_str(), reverse_complement);
    }

}
