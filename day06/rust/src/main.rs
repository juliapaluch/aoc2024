use std::{
    collections::HashSet, fs::File, io::{self, BufRead, BufReader, Seek}, ops::Add
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Pos {
    x: i16,
    y: i16,
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

const MIN_POS: i16 = 0;
const MAX_POS: i16 = 129;

fn part_one(file: &File) -> usize {
    let reader = BufReader::new(file);
    let mut guard_map: Vec<Vec<char>> = reader
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let cpos_y: i16 = guard_map
        .iter()
        .position(|x| x.contains(&'^')).unwrap() as i16;
    let cpos_x: i16 = guard_map[cpos_y as usize].iter().position(|x| x == &'^').unwrap() as i16;
    let mut cpos: Pos = Pos{x: cpos_x, y: cpos_y};
    guard_map[cpos.y as usize][cpos.x as usize] = '.';

    let mut distinct_positions = HashSet::new();
    let mut direction = 0;

    let positions = Vec::from([
        Pos{x: 0, y: -1},
        Pos{x: 1, y: 0},
        Pos{x: 0, y: 1},
        Pos{x: -1, y: 0},
    ]);
    
    while cpos.x > MIN_POS && cpos.x < MAX_POS && cpos.y > MIN_POS && cpos.y < MAX_POS {
        let npos = cpos + positions[direction];
        let obstacle = guard_map[npos.y as usize][npos.x as usize] == '#';
        if obstacle {
            if direction == 3 {
                direction = 0
            } else {
                direction += 1;
            }
        } else {
            cpos = npos;
            distinct_positions.insert(cpos);
        }
    }
    distinct_positions.len()
}

fn main() -> io::Result<()> {
    let mut file = File::open("src/input.txt")?;
    let p1 = part_one(&file);
    println!("Part 1: {}", p1);
    file.rewind()?;
    // let p2 = part_two(&file);
    // println!("Part 2: {}", p2 - p1);
    Ok(())
}
