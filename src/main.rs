use std::io::{self, BufRead};

fn main() {
    let input = io::stdin();
    let mut result = 0;
    for line in input.lock().lines() {
        let mut line_str = line.unwrap_or_default();
        result += line_str.parse::<i32>().unwrap_or_default();
    }
    println!("Result Day1: {:?}", result);
}
