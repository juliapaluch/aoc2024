use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn is_safe(s: &Vec<i64>) -> bool {
    let ord = if s[0] > s[1] {
        Ordering::Greater
    } else {
        Ordering::Less
    };
    s.windows(2).all(|s|(s[0] - s[1]).abs() < 4 && ord == s[0].cmp(&s[1]))
}

fn part_one(file: &File) -> usize {
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|s| is_safe(s))
        .count()
}

fn part_two(file: &File) -> usize {
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|x| {
            x.unwrap()
                .split_whitespace()
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|s| {
            let mut safe_with_dampened = false;
            for k in 0..s.len() {
                let mut c = s.clone();
                c.remove(k);
                if is_safe(&c) {
                    safe_with_dampened = true;
                    break;
                }
            }
            safe_with_dampened
        })
        .count()
}

fn main() -> io::Result<()> {
    let mut file = File::open("src/input.txt")?;
    println!("Part 1: {}", part_one(&file));
    file.rewind()?;
    println!("Part 2: {}", part_two(&file));
    Ok(())
}
