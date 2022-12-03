#!/usr/bin/env rust-script

use std::collections::HashSet;
use std::io;
use std::iter::FromIterator;

fn main() {
    let mut score = 0;

    for line in io::stdin().lines().map(|line| line.unwrap()) {
        let len = line.len() / 2;
        let first = HashSet::<_>::from_iter(line.chars().take(len));
        let second = HashSet::<_>::from_iter(line.chars().skip(len));

        for ch in first.intersection(&second) {
            match ch {
                'a'..='z' => score += u32::from(*ch) - 96,
                'A'..='Z' => score += u32::from(*ch) - 38,
                _ => panic!("Unexpected character!"),
            }
        }
    }

    println!("{}", score);
}
