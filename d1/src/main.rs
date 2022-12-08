use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let elves = find_elf_cals();
    println!("{}", elves.last().unwrap());
    let len = elves.len();
    println!("{}", elves[len - 1] + elves[len - 2] + elves[len - 3])
}

fn find_elf_cals() -> Vec<i32> {
    let file = File::open("inp.txt").unwrap();
    let reader = BufReader::new(file);

    let mut elves: Vec<i32> = reader.lines().fold(vec![0], |mut acc: Vec<i32>, val| {
        match val.unwrap().parse::<i32>() {
            Ok(e) => {
                if let Some(last) = acc.last_mut() {
                    *last += e;
                }
                acc
            }
            Err(_) => {
                acc.push(0);
                acc
            }
        }
    });

    elves.sort();
    elves
}
