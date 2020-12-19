use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct LineReader {
    reader: BufReader<File>,
}

impl LineReader {
    pub fn new(filename: &str) -> Self {
        let file =
            File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
        let reader = BufReader::new(file);
        Self { reader }
    }

    pub fn read_with<F>(&mut self, mut f: F)
    where
        F: FnMut(&str),
    {
        let mut line = String::new();
        loop {
            let bytes = self
                .reader
                .read_line(&mut line)
                .expect("Failed to read line");
            if bytes == 0 {
                break;
            }

            f(line.trim());

            line.clear();
        }
    }
}
