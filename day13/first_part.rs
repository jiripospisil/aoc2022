#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! nom = "7.1.1"
//! ```

#![feature(iter_array_chunks)]

use std::cmp::Ordering;
use std::io;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq, Clone)]
struct Packet {
    data: Data,
}

#[derive(Debug, PartialEq, Clone)]
enum Data {
    List(Vec<Data>),
    Integer(u32),
}

fn parse_list(i: &str) -> IResult<&str, Data> {
    let list_parser = delimited(tag("["), separated_list0(tag(","), parse_list), tag("]"))
        .map(|out| Data::List(out));
    let number_parser = complete::u32.map(|num| Data::Integer(num));

    alt((list_parser, number_parser))(i)
}

fn parse_packet(i: &str) -> IResult<&str, Packet> {
    let (i, result) = parse_list(i)?;

    Ok((i, Packet { data: result }))
}

fn is_in_right_order(first: &Packet, second: &Packet) -> bool {
    fn is_in_right_order_private(f: &Data, s: &Data) -> Ordering {
        match (f, s) {
            (Data::List(fv), Data::List(sv)) => {
                let mut fvi = fv.iter();
                let mut svi = sv.iter();

                for _ in 0..=(fv.len().min(sv.len())) {
                    let l = fvi.next();
                    let r = svi.next();

                    let res = match (l, r) {
                        (Some(ld), Some(rd)) => is_in_right_order_private(ld, rd),
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        _ => continue,
                    };

                    if res != Ordering::Equal {
                        return res;
                    }
                }

                Ordering::Equal
            }

            (Data::Integer(fi), Data::Integer(si)) => fi.cmp(si),

            (Data::Integer(fi), list @ Data::List(_)) => {
                is_in_right_order_private(&Data::List(vec![Data::Integer(*fi)]), list)
            }

            (list @ Data::List(_), Data::Integer(si)) => {
                is_in_right_order_private(list, &Data::List(vec![Data::Integer(*si)]))
            }
        }
    }

    match is_in_right_order_private(&first.data, &second.data) {
        Ordering::Equal => panic!("Unable to make a decision"),
        Ordering::Less => true,
        Ordering::Greater => false,
    }
}

fn main() {
    let sum = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .array_chunks::<3>()
        .enumerate()
        .fold(0usize, |acc, (idx, chunk)| {
            let (_, first) = parse_packet(&chunk[0]).unwrap();
            let (_, second) = parse_packet(&chunk[1]).unwrap();

            if is_in_right_order(&first, &second) {
                acc + idx + 1
            } else {
                acc
            }
        });

    println!("{sum}");
}
