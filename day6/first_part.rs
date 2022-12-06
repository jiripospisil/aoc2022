#!/usr/bin/env rust-script

#![feature(iter_collect_into)]

use std::collections::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    let chars = line.chars().collect::<Vec<_>>();
    for window in chars.windows(4).enumerate() {
        let mut set: HashSet<char> = HashSet::new();
        window.1.iter().collect_into(&mut set);

        if set.len() == 4 {
            println!("{}", window.0 + 4);
            break;
        }
    }

    Ok(())
}
