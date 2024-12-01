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

  let mut l3: Vec<i64> = Vec::new();
  for (l1, l2) in l1.iter().zip(l2.iter()) {
    l3.push((l1 - l2).abs());
  }
  let sum: i64 = l3.iter().sum();
  println!("Part 1: {}", sum);

  Ok(())
}