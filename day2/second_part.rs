#!/usr/bin/env rust-script

use std::collections::HashMap;
use std::io;

fn main() {
    let table = HashMap::from([
        (('A', 'X'), (1 + 3, 'Z')),
        (('A', 'Y'), (2 + 6, 'X')),
        (('A', 'Z'), (3 + 0, 'Y')),
        (('B', 'X'), (1 + 0, 'X')),
        (('B', 'Y'), (2 + 3, 'Y')),
        (('B', 'Z'), (3 + 6, 'Z')),
        (('C', 'X'), (1 + 6, 'Y')),
        (('C', 'Y'), (2 + 0, 'Z')),
        (('C', 'Z'), (3 + 3, 'X')),
    ]);

    let mut score = 0;

    for line in io::stdin().lines().map(|line| line.unwrap()) {
        let mut chars = line.chars();

        let first = chars.next().expect("Invalid line");
        let second = chars.skip(1).next().expect("Invalid line");
        let second = table
            .get(&(first, second))
            .expect("Missing key-value pair")
            .1;

        score += table
            .get(&(first, second))
            .expect("Missing key-value pair")
            .0;
    }

    println!("{}", score);
}
