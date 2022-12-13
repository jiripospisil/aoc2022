#!/usr/bin/env rust-script

use std::io;
use std::mem;
use std::str::FromStr;

const MONKEYS_COUNT: usize = 8;

#[derive(Debug, Copy, Clone)]
enum NewValue {
    Old,
    New(i32),
}

#[derive(Debug)]
struct Monkey {
    inspected: u32,
    items: Vec<i32>,
    new_operator: String,
    new_value: NewValue,
    test: i32,
    test_true: usize,
    test_false: usize,
}

fn parse_monkeys() -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut lines = io::stdin().lines().map(|line| line.unwrap()).into_iter();

    for _ in 0..MONKEYS_COUNT {
        lines.next().unwrap(); // Monkey number

        let mut items = Vec::new();
        let new_operator;
        let mut new_value = NewValue::Old;
        let test;
        let test_true;
        let test_false;

        {
            // Starting items
            let line: String = lines.next().unwrap();
            for piece in line.split(&[',', ' '][..]) {
                if let Ok(item) = i32::from_str(piece) {
                    items.push(item);
                }
            }
        }

        {
            // Operation
            let line = lines.next().unwrap();
            let operation: Vec<_> = line.split(" = ").collect();
            let pieces: Vec<_> = operation[1].split(" ").collect();
            new_operator = pieces[1].to_string();
            if let Ok(val) = i32::from_str(pieces[2]) {
                new_value = NewValue::New(val);
            }
        }

        {
            // Test
            let line: String = lines.next().unwrap();
            let pieces: Vec<_> = line.split(&[',', ' '][..]).collect();
            test = i32::from_str(pieces.last().unwrap()).unwrap();
        }

        {
            // If true
            let line: String = lines.next().unwrap();
            let pieces: Vec<_> = line.split(&[',', ' '][..]).collect();
            test_true = usize::from_str(pieces.last().unwrap()).unwrap();
        }

        {
            // If false
            let line: String = lines.next().unwrap();
            let pieces: Vec<_> = line.split(&[',', ' '][..]).collect();
            test_false = usize::from_str(pieces.last().unwrap()).unwrap();
        }

        monkeys.push(Monkey {
            inspected: 0,
            items,
            new_operator,
            new_value,
            test,
            test_true,
            test_false,
        });

        lines.next(); // New line
    }

    monkeys
}

fn new_op(new_operator: &String, new_value: NewValue, item: i32) -> i32 {
    let new_value = match new_value {
        NewValue::New(value) => value,
        NewValue::Old => item,
    };

    match new_operator.as_str() {
        "+" => item + new_value,
        "*" => item * new_value,
        _ => panic!("Unsupported operation"),
    }
}

fn process_round(monkey_idx: usize, monkeys: &mut Vec<Monkey>) {
    let items = mem::take(&mut monkeys[monkey_idx].items);
    monkeys[monkey_idx].inspected += items.len() as u32;

    for item in items {
        let monkey = &monkeys[monkey_idx];

        let new_worry_level = new_op(&monkey.new_operator, monkey.new_value, item);
        let new_worry_level = new_worry_level / 3;

        let address = if new_worry_level % monkey.test == 0 {
            monkey.test_true
        } else {
            monkey.test_false
        };

        monkeys[address].items.push(new_worry_level);
    }
}

fn main() {
    let mut monkeys = parse_monkeys();

    for _ in 0..20 {
        for monkey_idx in 0..MONKEYS_COUNT {
            process_round(monkey_idx, &mut monkeys);
        }
    }

    monkeys.sort_unstable_by(|a, b| a.inspected.cmp(&b.inspected));

    println!(
        "{}",
        monkeys[monkeys.len() - 1].inspected * monkeys[monkeys.len() - 2].inspected
    );
}
