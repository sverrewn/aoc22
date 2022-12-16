use std::collections::BTreeSet;

use std::fs::File;
use std::io::{BufRead, BufReader};



fn main() {
    println!("{}", shared_item_sum());
    println!("{}", badge_sum());
}

fn badge_sum() -> i32 {
    let file = File::open("inp.txt").unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut sets = vec![BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];

    let mut sum = 0;

    'l:loop {
        for set in &mut sets {
            if let Some(line) = lines.next() {
                set.append(&mut line.unwrap().chars().collect::<BTreeSet<char>>());
            }
            else {
                break 'l;
            }
        }
        sum += get_priority(&sets.iter().cloned().reduce(|accum, item| accum.intersection(&item).cloned().collect::<BTreeSet<char>>()).unwrap().pop_first().unwrap());
        for set in &mut sets {
            set.clear();
        }
    }

    sum
}

fn shared_item_sum() -> i32{
    let file = File::open("inp.txt").unwrap();
    let reader = BufReader::new(file);

    let mut set0 = BTreeSet::new();
    let mut set1 = BTreeSet::new();

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let len = line.len();
        let items = line.split_at(len / 2);

        set0.append(&mut items.0.chars().collect::<BTreeSet<char>>());
        set1.append(&mut items.1.chars().collect::<BTreeSet<char>>());

        let intersection = set0.intersection(&set1).next().unwrap();
        sum += get_priority(intersection);

        set0.clear();
        set1.clear();
    }
    sum
}

fn get_priority(char: &char) -> i32 {
    if char.is_ascii_lowercase() {
        *char as i32 - 96
    }
    else {
        *char as i32 - 38
    }
}