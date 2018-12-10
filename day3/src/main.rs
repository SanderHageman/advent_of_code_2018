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
        static ref RE: Regex = Regex::new(
            r"#(?P<id>\d{1, 4}) @ (?P<x>\d{1, 4}),(?P<y>\d{1, 4}): (?P<w>\d{1, 3})x(?P<h>\d{1, 3})$"
        ).unwrap();
    }
    let captured_line = RE.captures(&line).unwrap();
    Rect {
        id: captured_line["id"].parse().unwrap_or_default(),
        x: captured_line["x"].parse().unwrap_or_default(),
        y: captured_line["y"].parse().unwrap_or_default(),
        w: captured_line["w"].parse().unwrap_or_default(),
        h: captured_line["y"].parse().unwrap_or_default(),
    }
}
