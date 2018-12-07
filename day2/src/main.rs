use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut twos = 0;
    let mut threes = 0;

    let mut ids = Vec::new();

    let input = io::stdin();
    for line in input.lock().lines() {
        let mut letter_count: HashMap<char, i32> = HashMap::new();
        let line_str = line.unwrap_or_default();
        for ch in line_str.chars() {
            let counter = letter_count.entry(ch).or_default();
            *counter += 1;
        }

        ids.push(line_str);

        for n in &letter_count {
            if n.1 == &2 {
                twos += 1;
                break;
            }
        }

        for n in letter_count {
            if n.1 == 3 {
                threes += 1;
                break;
            }
        }
    }

    println!("Result Day2_1: {:?}", twos * threes);
    println!("Result Day2_2: {:?}", part2(&ids));
}

fn part2(ids: &Vec<String>) -> (String, String) {
    let eqcount = ids[0].len() - 1;
    for i in 0..ids.len() {
        for j in i+1..ids.len(){
            let zipped =
                ids[i].chars()
                .zip(ids[j].chars());
            let count = zipped
                .filter(|&(a, b)| a == b )
                .count();

            if count == eqcount{
                return (ids[i].clone(), ids[j].clone());
            }
        }
    }
    (String::from("Error"), String::from("Error"))
}
