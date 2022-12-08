#!/usr/bin/env rust-script

use std::io;

const NUM: usize = 99;

fn main() {
    let mut grid = [[-1i8; NUM]; NUM];
    let mut grid_score = [[1u32; NUM]; NUM];

    for (i, line) in io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .into_iter()
        .enumerate()
    {
        for (j, charr) in line.chars().enumerate() {
            grid[i][j] = (charr as u8 - 48) as i8;
        }
    }

    for num in 1..10 {
        let mut grid_curr = grid.clone();

        for i in 0..NUM {
            for j in 0..NUM {
                grid_curr[i][j] = grid_curr[i][j] - num;
            }
        }

        // // Left
        for i in 1..NUM - 1 {
            let mut score = 1;

            for j in 1..NUM - 1 {
                if grid_curr[i][j] < 0 {
                    score += 1;
                }

                if grid_curr[i][j] == 0 {
                    if grid_curr[i][j - 1] >= grid_curr[i][j] {
                        grid_score[i][j] *= 1;
                    } else {
                        grid_score[i][j] *= score;
                    }
                    score = 1;
                }
            }
        }

        // Right
        for i in 1..NUM - 1 {
            let mut score = 1;

            for j in (1..NUM - 1).rev() {
                if grid_curr[i][j] < 0 {
                    score += 1;
                }

                if grid_curr[i][j] == 0 {
                    if grid_curr[i][j + 1] >= grid_curr[i][j] {
                        grid_score[i][j] *= 1;
                    } else {
                        grid_score[i][j] *= score;
                    }
                    score = 1;
                }
            }
        }

        // Top
        for j in 1..(NUM - 1) {
            let mut score = 1;

            for i in 1..(NUM - 1) {
                if grid_curr[i][j] < 0 {
                    score += 1;
                }

                if grid_curr[i][j] == 0 {
                    if grid_curr[i - 1][j] >= grid_curr[i][j] {
                        grid_score[i][j] *= 1;
                    } else {
                        grid_score[i][j] *= score;
                    }
                    score = 1;
                }
            }
        }

        // Bottom
        for j in 1..(NUM - 1) {
            let mut score = 1;

            for i in (1..NUM - 1).rev() {
                if grid_curr[i][j] < 0 {
                    score += 1;
                }

                if grid_curr[i][j] == 0 {
                    if grid_curr[i + 1][j] >= grid_curr[i][j] {
                        grid_score[i][j] *= 1;
                    } else {
                        grid_score[i][j] *= score;
                    }
                    score = 1;
                }
            }
        }
    }

    let mut max = 0;
    for i in 1..(NUM - 1) {
        for j in 1..(NUM - 1) {
            if grid_score[i][j] > max {
                max = grid_score[i][j];
            }
        }
    }

    println!("{max}");
}
