use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Seek},
    thread::current,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct Antenna {
    location: Point,
    frequency: char,
}

const MAX_X: i16 = 49;
const MAX_Y: i16 = 49;

fn part_one(file: &File) -> usize {
    let reader: BufReader<&File> = BufReader::new(file);
    let antennas: Vec<Antenna> = reader
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.unwrap()
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '.' => None,
                    _ => Some(Antenna {
                        location: Point {
                            x: x as i16,
                            y: y as i16,
                        },
                        frequency: c,
                    }),
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<Antenna>>();

    let mut antinodes: HashSet<Point> = HashSet::<Point>::new();

    antennas.iter().for_each(|current_antenna: &Antenna| {
        antennas
            .iter()
            .filter(|other_antenna| {
                other_antenna.frequency == current_antenna.frequency
                    && other_antenna.location != current_antenna.location
            })
            .for_each(|other_antenna: &Antenna| {
                let diff = Point {
                    x: other_antenna.location.x - current_antenna.location.x,
                    y: other_antenna.location.y - current_antenna.location.y,
                };
                let next = Point {
                    x: other_antenna.location.x + diff.x,
                    y: other_antenna.location.y + diff.y,
                };
                if valid(next) {
                    antinodes.insert(next);
                }
                let previous = Point {
                    x: current_antenna.location.x + (-1 * diff.x),
                    y: current_antenna.location.y + (-1 * diff.y),
                };
                if valid(previous) {
                    antinodes.insert(previous);
                }
            })
    });
    antinodes.len()
}

fn part_two(file: &File) -> usize {
    let reader: BufReader<&File> = BufReader::new(file);
    let antennas: Vec<Antenna> = reader
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.unwrap()
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '.' => None,
                    _ => Some(Antenna {
                        location: Point {
                            x: x as i16,
                            y: y as i16,
                        },
                        frequency: c,
                    }),
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<Antenna>>();

    let mut antinodes: HashSet<Point> = HashSet::<Point>::new();

    antennas.iter().for_each(|current_antenna: &Antenna| {
        antennas
            .iter()
            .filter(|other_antenna| {
                other_antenna.frequency == current_antenna.frequency
                    && other_antenna.location != current_antenna.location
            })
            .for_each(|other_antenna: &Antenna| {
                antinodes.insert(current_antenna.location);
                antinodes.insert(other_antenna.location);
                let diff = Point {
                    x: other_antenna.location.x - current_antenna.location.x,
                    y: other_antenna.location.y - current_antenna.location.y,
                };
                let mut next = Point {
                    x: other_antenna.location.x + diff.x,
                    y: other_antenna.location.y + diff.y,
                };
                while valid(next) {
                    antinodes.insert(next);
                    next = Point {
                        x: next.x + diff.x,
                        y: next.y + diff.y,
                    };
                }
                let mut previous = Point {
                    x: current_antenna.location.x + (-1 * diff.x),
                    y: current_antenna.location.y + (-1 * diff.y),
                };
                while valid(previous) {
                    antinodes.insert(previous);
                    previous = Point {
                        x: previous.x + (-1 * diff.x),
                        y: previous.y + (-1 * diff.y),
                    };
                }
            })
    });
    antinodes.len()
}

fn valid(p: Point) -> bool {
    p.x >= 0 && p.x <= MAX_X && p.y >= 0 && p.y <= MAX_Y
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
