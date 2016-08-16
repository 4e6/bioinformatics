//!
//! Utilities to work with datasets
//!

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Dataset {
    contents: String,
}

impl Dataset {

    pub fn open_text<P: AsRef<Path>>(path: P) -> Dataset {
        let mut file = Dataset::open_file(path);
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Dataset { contents: contents }
    }

    pub fn open_fasta<P: AsRef<Path>>(path: P) -> Dataset {
        let raw = Dataset::open_text(path);
        let lines = raw.contents
            .lines()
            .skip(1)
            .fold(String::with_capacity(10usize.pow(7)), |s, line| s + line);

        Dataset { contents: lines }
    }

    pub fn contents(&self) -> &str {
        self.contents.as_str()
    }

    pub fn lines(&self) -> Vec<&str> {
        self.contents.lines().collect()
    }

    fn open_file<P: AsRef<Path>>(path: P) -> File {
        let display = path.as_ref().display();
        match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        }
    }

}

#[cfg(test)]
mod tests {

    use test::Bencher;

    static SALMONELLA_ENTERICA: &'static str = "data/Salmonella_enterica.txt";

    #[bench]
    fn bench_open_text(b: &mut Bencher) {
        b.iter(|| super::Dataset::open_text(SALMONELLA_ENTERICA));
    }

    #[bench]
    fn bench_open_fasta(b: &mut Bencher) {
        b.iter(|| super::Dataset::open_fasta(SALMONELLA_ENTERICA));
    }

}
