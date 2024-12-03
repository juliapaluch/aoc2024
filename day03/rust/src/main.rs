use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_one(file: &File) -> i64 {
    let reader = BufReader::new(file);
    let line = reader.lines().nth(0).unwrap().unwrap();
    let re = Regex::new(r"mul\((?<l>\d{1,3}),(?<r>\d{1,3})\)").unwrap();
    re.captures_iter(&line)
        .map(|c| {
            c.name("l").unwrap().as_str().parse::<i64>().unwrap()
                * c.name("r").unwrap().as_str().parse::<i64>().unwrap()
        })
        .sum()
}

fn main() -> io::Result<()> {
    let mut file = File::open("src/input.txt")?;
    println!("Part 1: {}", part_one(&file));
    // file.rewind()?;
    // println!("Part 2: {}", part_two(&file));
    Ok(())
}
