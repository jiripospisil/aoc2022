#!/usr/bin/env rust-script

use std::io;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn process_bound(b: &Option<&str>) -> u32 {
    let b = b.expect("Missing piece");
    u32::from_str(b).expect("Invalid range")
}

fn process_range(s: &Option<&str>, e: &Option<&str>) -> RangeInclusive<u32> {
    let s = process_bound(&s);
    let e = process_bound(&e);

    s..=e
}

fn process_line(line: &String) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let mut pieces = line.split(&['-', ','][..]);
    let lrange = process_range(&pieces.next(), &pieces.next());
    let rrange = process_range(&pieces.next(), &pieces.next());

    (lrange, rrange)
}

fn main() {
    let mut score = 0;

    for line in io::stdin().lines().map(|line| line.unwrap()) {
        let (lrange, rrange) = process_line(&line);

        if lrange.start() <= rrange.start() && lrange.end() >= rrange.start()
            || lrange.start() >= rrange.start() && lrange.start() <= rrange.end()
        {
            score += 1;
        }
    }

    println!("{}", score);
}
