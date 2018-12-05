use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let input = io::stdin();
    let mut frequencies = HashSet::new();
    let mut first_result = 0;
    let mut t_freq = 0;
    let mut changes: Vec<i32> = Vec::new();
    let mut second_result: Option<i32> = None;

    for line in input.lock().lines() {
        let line_str = line.unwrap_or_default();
        let change = line_str.parse::<i32>().unwrap_or_default();

        first_result += change;
        changes.push(change);
    }

    if second_result.is_none() {
        loop {
            for _change in changes.iter() {
                t_freq += _change;
                if !frequencies.insert(t_freq) {
                    if second_result.is_none() {
                        second_result = Some(t_freq);
                        break;
                    }
                }
            }

            if second_result.is_some() {
                break;
            }
        }
    }

    println!("Result Day1_1: {:?}", first_result);
    println!("Result Day1_2: {:?}", second_result);
}
