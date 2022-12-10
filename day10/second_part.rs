#!/usr/bin/env rust-script

use std::io;
use std::str::FromStr;

fn main() {
    let mut x = 1;
    let mut xs = vec![x];

    for line in io::stdin().lines().map(|line| line.unwrap()) {
        let split: Vec<_> = line.split(" ").collect();
        let instruction = split[0];

        if instruction == &"addx"[..] {
            let value = i32::from_str(split[1]).unwrap();

            xs.push(x);
            x += value;
            xs.push(x);
        } else {
            xs.push(x);
        }
    }

    // The whole image is rotated by one column for reasons :-/
    for i in 1usize..=240 {
        let col = i as i32 % 40;

        if col == xs[i] || col == xs[i] - 1 || col == xs[i] + 1 {
            print!("#");
        } else {
            print!(".");
        }

        if col == 0 {
            println!();
        }
    }
}
