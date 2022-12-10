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
            xs.push(x);

            let value = i32::from_str(split[1]).unwrap();
            x += value;
            xs.push(x);
        } else {
            xs.push(x);
        }
    }

    let mut result = 0;

    for i in (19..=219).step_by(40) {
        result += (i + 1) as i32 * xs[i];
    }

    println!("{result}");
}
