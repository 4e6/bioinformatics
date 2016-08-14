//!
//! Utilities to work with datasets
//!

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::Lines;

static DIR: &'static str = "data";

pub struct Dataset {
    contents: String,
}

impl Dataset {

    pub fn open(problem: &str, dataset: &str) -> Dataset {
        let path = Dataset::root_path(problem, dataset);
        let mut file = Dataset::open_file(path);
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        Dataset { contents: contents }
    }

    pub fn contents(&self) -> &str {
        self.contents.as_str()
    }

    pub fn lines<'a>(&'a self) -> Lines<'a> {
        self.contents.lines()
    }

    fn root_path(problem: &str, dataset: &str) -> PathBuf {
        let mut path = PathBuf::from(DIR);
        path.push(problem);
        path.push(dataset);
        path
    }

    fn open_file<P: AsRef<Path>>(path: P) -> File {
        let display = path.as_ref().display();
        match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        }
    }

}
