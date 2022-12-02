#!/usr/bin/env rust-script

use std::io;

fn main() -> io::Result<()> {
    let mut current = 0;
    let mut elves = Vec::new();

    for line in io::stdin().lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            elves.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().expect("Not a number");
        }
    }

    elves.sort_unstable();
    println!("{}", elves[elves.len() - 3..].into_iter().sum::<i32>());

    Ok(())
}
