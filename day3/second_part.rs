#!/usr/bin/env rust-script

#![feature(iter_array_chunks)]

use std::collections::HashSet;
use std::io;
use std::iter::FromIterator;

fn main() {
    let mut score = 0;

    for chunk in io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .array_chunks::<3>()
    {
        let first = HashSet::<_>::from_iter(chunk[0].chars());
        let second = HashSet::<_>::from_iter(chunk[1].chars());
        let third = HashSet::<_>::from_iter(chunk[2].chars());

        let intersection = first.intersection(&second);
        let intersection = HashSet::from_iter(intersection.cloned().collect::<Vec<_>>());

        for ch in intersection.intersection(&third) {
            match ch {
                'a'..='z' => score += u32::from(*ch) - 96,
                'A'..='Z' => score += u32::from(*ch) - 38,
                _ => panic!("Unexpected character!"),
            }
        }
    }

    println!("{}", score);
}
