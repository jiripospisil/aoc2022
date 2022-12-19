#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! itertools = "0.10.5"
//! ```

use std::fmt;
use std::io;
use std::str;

use itertools::Itertools;

const X: usize = 900;
const Y: usize = 200;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Material {
    Rock,
    Air,
    Sand,
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Material::Rock => write!(f, "#"),
            Material::Air => write!(f, "."),
            Material::Sand => write!(f, "o"),
        }
    }
}

fn is_valid(x: usize, y: usize) -> bool {
    x < X && y < Y
}

fn solve(grid: &mut [[Material; Y]; X]) -> u32 {
    for count in 0.. {
        let mut n = 500usize;
        if grid[500][0] == Material::Sand {
            return count;
        }

        for i in 0.. {
            if is_valid(n, i + 1) && matches!(grid[n][i + 1], Material::Rock | Material::Sand) {
                if is_valid(n - 1, i + 1) && grid[n - 1][i + 1] == Material::Air {
                    n -= 1;
                } else if is_valid(n + 1, i + 1) && grid[n + 1][i + 1] == Material::Air {
                    n += 1;
                } else {
                    grid[n][i] = Material::Sand;
                    break;
                }
            }
        }
    }

    panic!("BUG")
}

fn main() {
    let mut grid = [[Material::Air; Y]; X];

    let groups = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.split(" -> ")
                .map(|split| {
                    let mut split = split.split(",");
                    let x = str::parse::<usize>(split.next().unwrap()).unwrap();
                    let y = str::parse::<usize>(split.next().unwrap()).unwrap();
                    (x, y)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for group in &groups {
        for (t1, t2) in group.iter().tuple_windows() {
            if t1.0 == t2.0 {
                for i in t1.1.min(t2.1)..=t1.1.max(t2.1) {
                    grid[t1.0][i] = Material::Rock;
                }
            } else {
                for i in t1.0.min(t2.0)..=t1.0.max(t2.0) {
                    grid[i][t1.1] = Material::Rock;
                }
            }
        }
    }

    let max_y = groups
        .iter()
        .flat_map(|g| g.iter())
        .map(|t| t.1)
        .max()
        .unwrap()
        + 2;

    for x in 0..X {
        grid[x][max_y] = Material::Rock;
    }

    println!("{}", solve(&mut grid));
}
