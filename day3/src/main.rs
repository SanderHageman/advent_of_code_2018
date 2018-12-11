use std::collections::HashMap;
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
    let mut claims = Vec::new();
    let mut claimed_squares = HashMap::new();
    let input = io::stdin();
    for line in input.lock().lines() {
        claims.push(parse_line(line.unwrap()));
        println!("{:?}", claims.last());

        claim_squares(claims.last().unwrap(), &mut claimed_squares);
    }

    println!("Overlapping: {}", count_overlapping(&claimed_squares));
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

fn claim_squares(claim: &Rect, claimed_squares: &mut HashMap<(i32, i32), bool>) {
    for y in claim.y..(claim.y + claim.h) {
        for x in claim.x..(claim.x + claim.w) {
            let mut square = claimed_squares.insert((x, y), false);
            match square.as_mut() {
                Some(v) => *v = true,
                None => {}
            }
        }
    }
}

fn count_overlapping(claimed_squares: &HashMap<(i32, i32), bool>) -> i32 {
    let mut count = 0;

    for val in claimed_squares.values() {
        if *val {
            count += 1;
        }
    }

    count
}
