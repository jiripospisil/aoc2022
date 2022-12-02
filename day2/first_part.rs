#!/usr/bin/env rust-script

use std::collections::HashMap;
use std::io;

fn main() {
    let table = HashMap::from([
        (('A', 'X'), 1 + 3),
        (('A', 'Y'), 2 + 6),
        (('A', 'Z'), 3 + 0),
        (('B', 'X'), 1 + 0),
        (('B', 'Y'), 2 + 3),
        (('B', 'Z'), 3 + 6),
        (('C', 'X'), 1 + 6),
        (('C', 'Y'), 2 + 0),
        (('C', 'Z'), 3 + 3),
    ]);
    let mut score = 0;

    for line in io::stdin().lines().map(|line| line.unwrap()) {
        let mut chars = line.chars();

        let first = chars.next().expect("Invalid line");
        let second = chars.skip(1).next().expect("Invalid line");

        score += table.get(&(first, second)).expect("Missing key-value pair");
    }

    println!("{}", score);
}
