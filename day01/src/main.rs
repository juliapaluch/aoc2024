use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);

    let mut l1: Vec<i64> = Vec::new();
    let mut l2: Vec<i64> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        l1.push(parts[0].parse::<i64>().unwrap());
        l2.push(parts[1].parse::<i64>().unwrap());
    }

    l1.sort();
    l2.sort();

    println!(
        "Part 1: {}",
        l1.iter()
            .zip(l2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<i64>()
    );

    println!(
        "Part 2: {}",
        l1.iter()
            .map(|x| x * l2.iter().filter(|&y| x == y).count() as i64)
            .sum::<i64>()
    );

    Ok(())
}
