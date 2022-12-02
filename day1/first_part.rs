#!/usr/bin/env rust-script

use std::io;

fn main() -> io::Result<()> {
    let mut max = 0;
    let mut current = 0;

    for line in io::stdin().lines().map(|line| line.unwrap()) {
        if line.is_empty() {
            current = 0;
        } else {
            current += line.parse::<i32>().expect("Not a number");
        }

        if current > max {
            max = current;
        }
    }

    println!("{}", max);

    Ok(())
}
