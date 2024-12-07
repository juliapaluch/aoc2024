use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Seek},
    ops::Add,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Pos {
    x: i16,
    y: i16,
    direction: usize,
}

impl Add for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
            direction: self.direction,
        }
    }
}

const POSITIONS: &[Pos] = &[
    Pos {
        x: 0,
        y: -1,
        direction: 0,
    },
    Pos {
        x: 1,
        y: 0,
        direction: 1,
    },
    Pos {
        x: 0,
        y: 1,
        direction: 2,
    },
    Pos {
        x: -1,
        y: 0,
        direction: 3,
    },
];

const MIN_POS: i16 = 0;
const MAX_POS: i16 = 129;

// 5409

fn part_one(file: &File) -> usize {
    let reader = BufReader::new(file);
    let mut guard_map: Vec<Vec<char>> = reader
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let cpos_y: i16 = guard_map.iter().position(|x| x.contains(&'^')).unwrap() as i16;
    let cpos_x: i16 = guard_map[cpos_y as usize]
        .iter()
        .position(|x| x == &'^')
        .unwrap() as i16;
    let mut cpos: Pos = Pos {
        x: cpos_x,
        y: cpos_y,
        direction: 0,
    };
    guard_map[cpos.y as usize][cpos.x as usize] = '.';

    let mut distinct_positions = HashSet::new();
    let mut direction = 0;

    while cpos.x > MIN_POS && cpos.x < MAX_POS && cpos.y > MIN_POS && cpos.y < MAX_POS {
        let npos = cpos + POSITIONS[direction];
        let obstacle = guard_map[npos.y as usize][npos.x as usize] == '#';
        let next_direction = if direction == 3 { 0 } else { direction + 1 };
        if obstacle {
            direction = next_direction;
        } else {
            cpos = npos;
            distinct_positions.insert(cpos);
        }
    }
    distinct_positions.len()
}

fn part_two(file: &File) -> usize {
    let reader = BufReader::new(file);
    let mut guard_map: Vec<Vec<char>> = reader
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let cpos_y: i16 = guard_map.iter().position(|x| x.contains(&'^')).unwrap() as i16;
    let cpos_x: i16 = guard_map[cpos_y as usize]
        .iter()
        .position(|x| x == &'^')
        .unwrap() as i16;
    let mut cpos: Pos = Pos {
        x: cpos_x,
        y: cpos_y,
        direction: 0,
    };
    let opos = cpos;
    guard_map[cpos.y as usize][cpos.x as usize] = '.';

    let mut distinct_positions = HashSet::new();
    distinct_positions.insert(cpos);
    let mut test_distinct_positions: HashSet<Pos> = HashSet::new();
    let mut obstacle_positions: HashSet<Pos> = HashSet::new();

    println!("Starting at cpos {:?}", cpos);

    while valid(cpos) {
        if peek(&guard_map, &cpos) != '#' && cpos + POSITIONS[cpos.direction] != opos {
            let mut ppos = cpos;
            test_distinct_positions.clear();
            test_distinct_positions.insert(ppos);
            let mut obstacle_position = ppos + POSITIONS[ppos.direction];
            ppos = turn_right(ppos);
            while valid(ppos) {
                ppos = advance(&guard_map, ppos);
                println!("Checking {:?}", ppos);
                if distinct_positions.contains(&ppos) || test_distinct_positions.contains(&ppos) {
                    // loop detected
                    obstacle_position.direction = 0;
                    obstacle_positions.insert(obstacle_position);
                    break;
                }
                test_distinct_positions.insert(ppos);
            }
        }
        cpos = advance(&guard_map, cpos);
        println!("New cpos {:?}", cpos);
        println!("Adding {:?} to distinct positions", cpos);
        distinct_positions.insert(cpos);
    }
    obstacle_positions.len()
}

fn turn_right(mut pos: Pos) -> Pos {
    pos.direction = if pos.direction == 3 {
        0
    } else {
        pos.direction + 1
    };
    pos
}

fn valid(pos: Pos) -> bool {
    pos.x > MIN_POS && pos.x < MAX_POS && pos.y > MIN_POS && pos.y < MAX_POS
}

fn advance(guard_map: &Vec<Vec<char>>, mut cpos: Pos) -> Pos {
    if peek(guard_map, &cpos) == '#' {
        cpos = turn_right(cpos);
        return cpos;
    }
    cpos + POSITIONS[cpos.direction]
}

fn peek(guard_map: &Vec<Vec<char>>, cpos: &Pos) -> char {
    let mut npos = *cpos + POSITIONS[cpos.direction];
    npos.direction = cpos.direction;
    guard_map[npos.y as usize][npos.x as usize]
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
