#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! regex = "1.7.0"
//! ```

use regex::{Match, Regex};
use std::collections::VecDeque;
use std::io;
use std::str::FromStr;

fn parse_num<'t>(m: &Option<Match<'t>>) -> usize {
    usize::from_str(&m.unwrap().as_str()).unwrap()
}

fn main() {
    let mut stacks: [VecDeque<String>; 9] = Default::default();
    let regex_data = Regex::new(r"\[[A-Z]\]").expect("Invalid regex");
    let regex_ins = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").expect("Invalid regex");
    let mut processing = false;

    for line in io::stdin().lines().map(|line| line.unwrap()) {
        if !processing {
            for mat in regex_data.find_iter(&line) {
                let as_str = mat.as_str();
                stacks[mat.start() / 4].push_front(as_str[1..as_str.len() - 1].to_owned());
            }
        }

        if let Some(ins) = regex_ins.captures(&line) {
            processing = true;

            let count = parse_num(&ins.get(1));
            let from = parse_num(&ins.get(2)) - 1;
            let to = parse_num(&ins.get(3)) - 1;

            let pos = stacks[from].len() - count;
            let taken: Vec<_> = stacks[from].drain(pos..).collect();
            stacks[to].extend(taken);
        }
    }

    for stack in stacks {
        if let Some(back) = stack.back() {
            print!("{}", back);
        }
    }

    println!("");
}
