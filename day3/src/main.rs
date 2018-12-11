use std::io::{self, BufRead};

#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

#[derive(Default, Debug)]
struct Rect {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

fn main() {
    let mut squares = Vec::new();
    let input = io::stdin();
    for line in input.lock().lines() {
        squares.push(parse_line(line.unwrap()));
        println!("{:?}", squares.last());
    }
}

fn parse_line(line: String) -> Rect {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    }
    let captured_line = RE.captures(&line).unwrap();
    Rect {
        id: captured_line[1].parse().unwrap_or_default(),
        x: captured_line[2].parse().unwrap_or_default(),
        y: captured_line[3].parse().unwrap_or_default(),
        w: captured_line[4].parse().unwrap_or_default(),
        h: captured_line[5].parse().unwrap_or_default(),
    }
}
