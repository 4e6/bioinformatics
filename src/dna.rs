use std::str;

pub struct DNA {
    pub seq: Vec<u8>
}

impl DNA {

    pub fn len(&self) -> usize {
        self.seq.len()
    }

    pub fn from_slice(s: &[u8]) -> DNA {
        let mut seq = Vec::new();
        seq.extend_from_slice(s);
        DNA { seq: seq }
    }

    pub fn from_str(s: &str) -> DNA {
        let bytes = s.as_bytes();
        DNA::from_slice(bytes)
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.seq.clone()).unwrap()
    }

    pub fn as_str(&self) -> &str {
        str::from_utf8(&self.seq).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::DNA;

    static SEQ_STRING: &'static str = "ABAC";

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
}
