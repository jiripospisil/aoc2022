#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! regex = "1.7.0"
//! itertools = "0.10.5"
//! ```

use std::collections::HashSet;
use std::io;

use itertools::Itertools;
use regex::Regex;

const Y: i32 = 2000000;

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

    let mut target_row = HashSet::new();

    for (sx, sy, bx, by) in &points {
        let d = distance(*sx, *sy, *bx, *by);

        if sy + d < Y || sy - d > Y {
            continue;
        }

        let syy = distance(*sx, *sy, *sx, Y);
        let crosses = d * 2 + 1 - syy * 2;
        let crosses = (crosses - 1) / 2;

        for i in sx - crosses..=sx + crosses {
            target_row.insert(i);
        }
    }

    let on_y = points
        .iter()
        .filter(|(_, _, _, by)| *by == Y)
        .unique_by(|(_, _, bx, by)| (bx, by))
        .fold(
            0usize,
            |acc, (_, _, _, by)| if *by == Y { acc + 1 } else { acc },
        );

    println!("{}", target_row.len() - on_y);
}
