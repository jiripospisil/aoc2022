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

#[derive(Clone)]
struct Packet {
    data: Data,
}

#[derive(Clone)]
enum Data {
    List(Vec<Data>),
    Integer(u32),
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        is_in_right_order(&self.data, &other.data)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn is_in_right_order(f: &Data, s: &Data) -> Ordering {
    match (f, s) {
        (Data::List(fv), Data::List(sv)) => {
            let mut fvi = fv.iter();
            let mut svi = sv.iter();

            for _ in 0..=(fv.len().min(sv.len())) {
                let l = fvi.next();
                let r = svi.next();

                let res = match (l, r) {
                    (Some(ld), Some(rd)) => is_in_right_order(ld, rd),
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
            is_in_right_order(&Data::List(vec![Data::Integer(*fi)]), list)
        }

        (list @ Data::List(_), Data::Integer(si)) => {
            is_in_right_order(list, &Data::List(vec![Data::Integer(*si)]))
        }
    }
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

fn main() {
    let mut packets: Vec<_> = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .array_chunks::<3>()
        .flat_map(|chunk| {
            let (_, first) = parse_packet(&chunk[0]).unwrap();
            let (_, second) = parse_packet(&chunk[1]).unwrap();

            [first, second]
        })
        .collect();

    let first = Packet {
        data: Data::List(vec![Data::List(vec![Data::Integer(2)])]),
    };
    packets.push(first.clone());

    let second = Packet {
        data: Data::List(vec![Data::List(vec![Data::Integer(6)])]),
    };
    packets.push(second.clone());

    packets.sort_unstable();

    let first_idx = packets.iter().position(|packet| packet == &first);
    let second_idx = packets.iter().position(|packet| packet == &second);

    println!("{}", (first_idx.unwrap() + 1) * (second_idx.unwrap() + 1));
}
