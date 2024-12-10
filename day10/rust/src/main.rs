use std::{
    collections::HashSet, fs::File, io::{self, BufRead, BufReader, Seek}
};

const MAX_X: i32 = 41;
const MAX_Y: i32 = 41;

fn part_one(file: &File) -> i32 {
    let reader: BufReader<&File> = BufReader::new(file);
    let map = reader
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|x| x.to_string().parse::<u8>().unwrap_or(9))
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == 0)
                .map(|(x, n)| {
                    let mut unique_peaks= HashSet::<(i32, i32)>::new();
                    num_reachable_peaks(&map, (x as i32 + 1, y as i32), *n, &mut unique_peaks);
                    num_reachable_peaks(&map, (x as i32, y as i32 + 1), *n, &mut unique_peaks);
                    num_reachable_peaks(&map, (x as i32 - 1, y as i32), *n, &mut unique_peaks);
                    num_reachable_peaks(&map, (x as i32, y as i32 - 1), *n, &mut unique_peaks);
                    unique_peaks.len() as i32
                })
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn part_two(file: &File) -> i32 {
    let reader: BufReader<&File> = BufReader::new(file);
    let map = reader
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|x| x.to_string().parse::<u8>().unwrap_or(9))
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == 0)
                .map(|(x, n)| {
                    num_reachable_paths(&map, (x as i32 + 1, y as i32), *n) +
                    num_reachable_paths(&map, (x as i32, y as i32 + 1), *n) +
                    num_reachable_paths(&map, (x as i32 - 1, y as i32), *n) +
                    num_reachable_paths(&map, (x as i32, y as i32 - 1), *n)
                })
                .sum::<i32>()
        })
        .sum::<i32>()
}

fn num_reachable_paths(map: &Vec<Vec<u8>>, pos: (i32, i32), previous_elevation: u8) -> i32 {
    if valid(pos) {
        if map[pos.1 as usize][pos.0 as usize] == previous_elevation + 1 {
            if map[pos.1 as usize][pos.0 as usize] == 9 {
                return 1;
            } else {
                return num_reachable_paths(map, (pos.0 + 1, pos.1), previous_elevation + 1)
                    + num_reachable_paths(map, (pos.0, pos.1 + 1), previous_elevation + 1)
                    + num_reachable_paths(map, (pos.0 - 1, pos.1), previous_elevation + 1)
                    + num_reachable_paths(map, (pos.0, pos.1 - 1), previous_elevation + 1);
            }
        } else {
            return 0;
        }
    } else {
        return 0;
    }
}

fn num_reachable_peaks(map: &Vec<Vec<u8>>, pos: (i32, i32), previous_elevation: u8, unique_peaks: &mut HashSet<(i32, i32)>) -> i32 {
    if valid(pos) {
        if map[pos.1 as usize][pos.0 as usize] == previous_elevation + 1 {
            if map[pos.1 as usize][pos.0 as usize] == 9 {
                unique_peaks.insert(pos);
                return 1;
            } else {
                return num_reachable_peaks(map, (pos.0 + 1, pos.1), previous_elevation + 1, unique_peaks)
                    + num_reachable_peaks(map, (pos.0, pos.1 + 1), previous_elevation + 1, unique_peaks)
                    + num_reachable_peaks(map, (pos.0 - 1, pos.1), previous_elevation + 1, unique_peaks)
                    + num_reachable_peaks(map, (pos.0, pos.1 - 1), previous_elevation + 1, unique_peaks);
            }
        } else {
            return 0;
        }
    } else {
        return 0;
    }
}

fn valid((x, y): (i32, i32)) -> bool {
    x >= 0 && x <= MAX_X && y >= 0 && y <= MAX_Y
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
