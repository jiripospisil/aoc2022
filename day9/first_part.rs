#!/usr/bin/env rust-script

use std::collections::HashSet;
use std::io;
use std::str::FromStr;

type Knot = [i32; 2];

fn get_new_tail(head: &Knot, tail: &Knot) -> Knot {
    let x = (head[0] - tail[0]).abs();
    let y = (head[1] - tail[1]).abs();

    if x <= 1 && y <= 1 {
        return *tail;
    }

    let mut new_tail = *tail;

    if head[0] > tail[0] {
        new_tail[0] += 1;
    }

    if head[0] < tail[0] {
        new_tail[0] -= 1;
    }

    if head[1] > tail[1] {
        new_tail[1] += 1;
    }

    if head[1] < tail[1] {
        new_tail[1] -= 1;
    }

    new_tail
}

fn main() {
    let mut head = Knot::default();
    let mut tail = Knot::default();
    let mut visited = HashSet::new();
    visited.insert(tail);

    for line in io::stdin().lines().map(|line| line.unwrap()) {
        let split: Vec<_> = line.split(" ").collect();
        let direction = split[0];
        let mut steps = u8::from_str(split[1]).unwrap() as i32;

        while steps > 0 {
            if direction == &"R"[..] {
                head[1] += 1;
            } else if direction == &"L"[..] {
                head[1] -= 1;
            } else if direction == &"U"[..] {
                head[0] += 1;
            } else if direction == &"D"[..] {
                head[0] -= 1;
            }

            let new_tail = get_new_tail(&head, &tail);

            if new_tail != tail {
                visited.insert(new_tail);
                tail = new_tail;
            }

            steps -= 1;
        }
    }

    println!("{}", visited.len());
}
