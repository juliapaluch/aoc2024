use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    println!(
        "Part 1: {:?}",
        reader
            .lines()
            .map(|x| {
                x.unwrap()
                    .split_whitespace()
                    .map(|y| y.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .filter(|s| {
                if s[0] > s[1] {
                    s[..]
                        .windows(2)
                        .all(|s| s[0] > s[1] && (1..4).contains(&(s[0] - s[1]).abs()))
                } else {
                    s[..]
                        .windows(2)
                        .all(|s| s[0] < s[1] && (1..4).contains(&(s[0] - s[1]).abs()))
                }
            })
            .count()
    );

    Ok(())
}
