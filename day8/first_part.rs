#!/usr/bin/env rust-script

use std::io;

const NUM: usize = 99 + 2;

fn main() {
    let mut grid = [[-1i8; NUM]; NUM];

    for (i, line) in io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .into_iter()
        .enumerate()
    {
        for (j, charr) in line.chars().enumerate() {
            grid[i + 1][j + 1] = (charr as u8 - 48) as i8;
        }
    }

    let mut visible = [[false; NUM]; NUM];

    // Left
    for i in 0..NUM {
        let mut max = grid[i][0];

        for j in 1..NUM {
            if grid[i][j] > max {
                visible[i][j] = true;
            }
            max = max.max(grid[i][j]);
        }
    }

    // Right
    for i in 0..NUM {
        let mut max = grid[i][NUM - 1];

        for j in (0..NUM).rev() {
            if grid[i][j] > max {
                visible[i][j] = true;
            }
            max = max.max(grid[i][j]);
        }
    }

    // Top
    for j in 0..NUM {
        let mut max = grid[0][j];

        for i in 1..NUM {
            if grid[i][j] > max {
                visible[i][j] = true;
            }
            max = max.max(grid[i][j]);
        }
    }

    // Bottom
    for j in (0..NUM).rev() {
        let mut max = grid[NUM - 1][j];

        for i in (0..NUM - 1).rev() {
            if grid[i][j] > max {
                visible[i][j] = true;
            }
            max = max.max(grid[i][j]);
        }
    }

    let mut visible_num = 0;

    for i in 0..NUM {
        for j in 0..NUM {
            if visible[i][j] {
                visible_num += 1;
            }
        }
    }

    println!("{}", visible_num);
}
