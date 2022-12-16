#!/usr/bin/env rust-script

use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::io;

const X: usize = 159;
const Y: usize = 41;

fn solve(grid: &[[u8; Y]; X], end: &(i32, i32)) -> usize {
    let mut to_check = VecDeque::new();
    to_check.push_back((*end, 0));

    let mut visited = HashSet::new();
    visited.insert(*end);

    while let Some(curr) = to_check.pop_back() {
        let start = &curr.0;
        let steps = &curr.1;

        for [x, y] in [[1, 0], [-1, 0], [0, 1], [0, -1]] {
            let new_start = (start.0 + x, start.1 + y);

            if new_start.0 < 0
                || new_start.1 < 0
                || new_start.0 >= X.try_into().unwrap()
                || new_start.1 >= Y.try_into().unwrap()
            {
                continue;
            }

            let point = grid[start.0 as usize][start.1 as usize];
            let point_new = grid[new_start.0 as usize][new_start.1 as usize];

            if point_new + 1 < point {
                continue;
            }

            if visited.contains(&new_start) {
                continue;
            }

            if point_new == 1 {
                return *steps + 1;
            }

            visited.insert(new_start);
            to_check.push_front((new_start, steps + 1));
        }
    }

    panic!("BUG");
}

fn main() {
    let mut grid = [[0u8; Y]; X];
    let mut end = (0i32, 0i32);

    for (idy, line) in io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .into_iter()
        .enumerate()
    {
        for (idx, ch) in line.chars().enumerate() {
            if ch == 'S' {
                grid[idx][idy] = 1;
            } else if ch == 'E' {
                end = (idx as i32, idy as i32);
                grid[idx][idy] = 26;
            } else {
                grid[idx][idy] = ch as u8 - 96;
            }
        }
    }

    for y in 0..Y {
        for x in 0..X {
            print!("{:3}", grid[x][y]);
        }
        println!();
    }

    println!("{}", solve(&grid, &end));
}
