use std::io::{self, BufRead};

fn main() {
    let input = io::stdin();
    let mut first_result = 0;

    for line in input.lock().lines() {
        let line_str = line.unwrap_or_default();
        println!("{}", line_str);
    }
}
