#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! regex = "1.7.0"
//! itertools = "0.10.5"
//! ```

use std::io;
use std::process;

use itertools::Itertools;
use regex::Regex;

const N: i32 = 4000000;

#[inline]
fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn main() {
    let numbers_regex = Regex::new(r"-?\d+").expect("Invalid regex");

    let points = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            numbers_regex
                .find_iter(&line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut ranges = Vec::new();

    for (sx, sy, bx, by) in &points {
        let d = distance(*sx, *sy, *bx, *by);

        for (idx, row) in (*sy - d..=*sy).enumerate() {
            if row >= 0 && row < N {
                ranges.push((row, *sx - idx as i32..=*sx + idx as i32));
            }
        }

        for (idx, row) in (*sy..=*sy + d).enumerate() {
            if row >= 0 && row < N {
                ranges.push((row, *sx - d + idx as i32..=*sx + d - idx as i32));
            }
        }
    }

    ranges.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.start().cmp(&b.1.start())
        } else {
            a.0.cmp(&b.0)
        }
    });

    for (_, group) in ranges.into_iter().group_by(|(row, _)| *row).into_iter() {
        let mut init = 0..=0;
        let mut start = true;

        for (row, range) in group {
            if start {
                start = false;
                init = *init.start().min(range.start())..=*init.end().max(range.end());
                continue;
            }

            if init.start() <= range.start() && init.end() >= range.end() {
                continue;
            }

            if init.end() + 1 < *range.start() {
                println!("{}", (*init.end() as u64 + 1) * 4000000 + row as u64);
                process::exit(0);
            }

            if range.start() < init.start() {
                init = *range.start()..=*init.end();
            }

            init = *init.start().min(range.start())..=*init.end().max(range.end());
        }
    }
}
