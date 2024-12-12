use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

fn part_one(file: &File) -> u128 {
    let mut reader: BufReader<&File> = BufReader::new(file);
    let mut stones = String::new();
    reader.read_line(&mut stones).unwrap();
    let mut stones: HashMap<u128, u128> = stones
        .split_whitespace()
        .map(|x| (x.parse::<u128>().unwrap(), 1))
        .collect::<HashMap<u128, u128>>();
    for _ in 0..25 {
        stones = blink(stones);
    }
    stones.iter().map(|(_, v)| v).sum::<u128>()
}

fn blink(stones: HashMap<u128, u128>) -> HashMap<u128, u128> {
    let mut new_stones: HashMap<u128, u128> = HashMap::new();
    for (stone, count) in stones {
        let mut new_vals: Vec<u128> = Vec::new();
        if stone == 0 {
            new_vals.push(1);
        } else if stone.ilog10() + 1 & 1 == 0 {
            let str_num = stone.to_string();
            new_vals.push(str_num[0..str_num.len() / 2].parse::<u128>().unwrap());
            new_vals.push(
                str_num[str_num.len() / 2..str_num.len()]
                    .parse::<u128>()
                    .unwrap(),
            );
        } else {
            new_vals.push(stone * 2024);
        }
        new_vals.iter().for_each(|stone| {
            if new_stones.contains_key(stone) {
                *new_stones.get_mut(stone).unwrap() += count;
            } else {
                new_stones.insert(*stone, count);
            }
        });
    }
    new_stones
}

fn part_two(file: &File) -> u128 {
    let mut reader: BufReader<&File> = BufReader::new(file);
    let mut stones = String::new();
    reader.read_line(&mut stones).unwrap();
    let mut stones: HashMap<u128, u128> = stones
        .split_whitespace()
        .map(|x| (x.parse::<u128>().unwrap(), 1))
        .collect::<HashMap<u128, u128>>();
    for _ in 0..75 {
        stones = blink(stones);
    }
    stones.iter().map(|(_, v)| v).sum::<u128>()
}

fn main() -> io::Result<()> {
    let mut file = File::open("src/input.txt")?;
    let p1 = part_one(&file);
    println!("Part 1: {}", p1);
    file.rewind()?;
    let p2 = part_two(&file);
    println!("Part 2: {}", p2);
    Ok(())
}
