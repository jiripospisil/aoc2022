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
    let mut knots = [Knot::default(); 10];
    let mut visited = HashSet::new();
    visited.insert(knots[9]);

    for line in io::stdin().lines().map(|line| line.unwrap()) {
        let split: Vec<_> = line.split(" ").collect();
        let direction = split[0];
        let mut steps = u8::from_str(split[1]).unwrap() as i32;

        while steps > 0 {
            let head = &mut knots[0];
            if direction == &"R"[..] {
                head[1] += 1;
            } else if direction == &"L"[..] {
                head[1] -= 1;
            } else if direction == &"U"[..] {
                head[0] += 1;
            } else if direction == &"D"[..] {
                head[0] -= 1;
            }

            for i in 0..9 {
                let new_tail = get_new_tail(&knots[i], &knots[i + 1]);

                if new_tail != knots[i + 1] {
                    knots[i + 1] = new_tail;
                    if i + 1 == 9 {
                        visited.insert(new_tail);
                    }
                }
            }

            steps -= 1;
        }
    }

    println!("{}", visited.len());
}
