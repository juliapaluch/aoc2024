use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
    ops::Add,
};

const XMAS: &[char] = &['X', 'M', 'A', 'S'];

#[derive(Clone, Copy)]
struct Pos {
    x: i16,
    y: i16,
}

const MIN_POS: i16 = 0;
// hard coded from input max line length
const MAX_POS: i16 = 139;

const DIRECTIONS: &[Pos] = &[
    Pos { x: -1, y: 1 },
    Pos { x: 0, y: 1 },
    Pos { x: 1, y: 1 },
    Pos { x: 1, y: 0 },
    Pos { x: 1, y: -1 },
    Pos { x: 0, y: -1 },
    Pos { x: -1, y: -1 },
    Pos { x: -1, y: 0 },
];

impl Add for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn find_match(grid: &Vec<Vec<char>>, cpos: Pos, index: usize, advance: Pos) -> i64 {
    if check_char(grid, cpos, XMAS[index]) {
        // we have a match
        if index == 3 {
            // and we're in the last possible XMAS state
            1
        } else {
            find_match(grid, cpos + advance, index + 1, advance)
        }
    } else {
        // no match
        0
    }
}

fn valid(cpos: Pos) -> bool {
    !(cpos.x > MAX_POS || cpos.x < MIN_POS || cpos.y > MAX_POS || cpos.y < MIN_POS)
}

fn find_xmas(grid: &Vec<Vec<char>>, cpos: Pos) -> bool {
    let xmas_directions: Vec<Pos> = Vec::from([
        Pos { x: 1, y: -1 },
        Pos { x: 1, y: 1 },
        Pos { x: -1, y: 1 },
        Pos { x: -1, y: -1 },
    ]);
    if check_char(grid, cpos, 'A') {
        let possible_mas: Vec<char> = xmas_directions
            .iter()
            .map(|p| get_char_at_pos(grid, cpos + *p))
            .collect();
        if possible_mas.iter().filter(|c| **c == 'M').count() == 2
            && possible_mas.iter().filter(|c| **c == 'S').count() == 2
        {
            // MAMSAS edge case
            !(possible_mas == ['S', 'M', 'S', 'M']) && !(possible_mas == ['M', 'S', 'M', 'S'])
        } else {
            false
        }
    } else {
        false
    }
}

fn get_char_at_pos(grid: &Vec<Vec<char>>, cpos: Pos) -> char {
    if !valid(cpos) {
        // this means we're in an invalid position in the grid so return X
        'X'
    } else {
        grid[usize::try_from(cpos.x).ok().unwrap()][usize::try_from(cpos.y).ok().unwrap()]
    }
}

fn check_char(grid: &Vec<Vec<char>>, cpos: Pos, c: char) -> bool {
    if !valid(cpos) {
        // this means we're in an invalid position in the grid so early exit
        false
    } else {
        grid[usize::try_from(cpos.x).ok().unwrap()][usize::try_from(cpos.y).ok().unwrap()] == c
    }
}

fn part_one(file: &File) -> i64 {
    let reader = BufReader::new(file);
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    grid.iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, _)| {
                    DIRECTIONS
                        .iter()
                        .map(|p| {
                            find_match(
                                &grid,
                                Pos {
                                    x: x.try_into().unwrap(),
                                    y: y.try_into().unwrap(),
                                },
                                0,
                                *p,
                            )
                        })
                        .sum::<i64>()
                })
                .sum::<i64>()
        })
        .sum::<i64>()
}

fn part_two(file: &File) -> usize {
    let reader = BufReader::new(file);
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    grid.iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, _)| {
                    find_xmas(
                        &grid,
                        Pos {
                            x: x.try_into().unwrap(),
                            y: y.try_into().unwrap(),
                        },
                    )
                })
                .filter(|b| *b)
                .count()
        })
        .sum::<usize>()
}

fn main() -> io::Result<()> {
    let mut file = File::open("src/input.txt")?;
    println!("Part 1: {}", part_one(&file));
    file.rewind()?;
    println!("Part 2: {}", part_two(&file));
    Ok(())
}
