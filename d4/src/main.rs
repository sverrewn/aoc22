use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("inp.txt").unwrap();
    let mut lines = BufReader::new(file).lines();
}