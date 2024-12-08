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
const MAX_X: i16 = 129;
const MAX_Y: i16 = 129;

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

    while cpos.x > MIN_POS && cpos.x < MAX_X && cpos.y > MIN_POS && cpos.y < MAX_Y {
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
    let mut obstacles: Vec<Pos> = reader
        .lines()
        .enumerate()
        .map(|(y, row)| {
            row.unwrap()
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '#' => Some(Pos {
                        x: x as i16,
                        y: y as i16,
                        direction: 4,
                    }),
                    '^' => Some(Pos {
                        x: x as i16,
                        y: y as i16,
                        direction: 0,
                    }),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<Pos>>();

    let starting_position =
        obstacles.remove(obstacles.iter().position(|p| p.direction == 0).unwrap());
    let mut current_position = starting_position;
    let mut possible_obstacles = HashSet::<Pos>::new();
    let mut visited_states = HashSet::<Pos>::new();

    while valid(current_position) {
        visited_states.insert(current_position);
        let next_position = move_forward(current_position);
        if obstacles
            .iter()
            .any(|p| p.x == next_position.x && p.y == next_position.y)
        {
            current_position = turn_right(current_position);
        } else {
            if valid(next_position)
                && !(next_position.x == starting_position.x
                    && next_position.y == starting_position.y)
            {
                let obstacle = Pos {
                    x: next_position.x,
                    y: next_position.y,
                    direction: 4,
                };
                if !possible_obstacles.contains(&obstacle) && !obstacles.contains(&obstacle) {
                    obstacles.push(obstacle);
                    if !possible_obstacles.contains(&obstacle)
                        && obstacle_results_in_infinite_loop(&obstacles, starting_position)
                    {
                        possible_obstacles.insert(obstacle);
                    }
                    obstacles.pop();
                }
            }
            current_position = next_position;
        }
    }

    possible_obstacles.len()
}

fn obstacle_results_in_infinite_loop(obstacles: &Vec<Pos>, starting_position: Pos) -> bool {
    let mut visited_states: HashSet<Pos> = HashSet::new();
    let mut current_position = starting_position;
    while valid(current_position) {
        let next_position = current_position + POSITIONS[current_position.direction];
        if obstacles
            .iter()
            .any(|p| p.x == next_position.x && p.y == next_position.y)
        {
            current_position = turn_right(current_position);
        } else {
            current_position = next_position;
        }
        if !visited_states.insert(current_position) {
            return true;
        }
    }
    false
}

fn move_forward(pos: Pos) -> Pos {
    pos + POSITIONS[pos.direction]
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
    pos.x >= MIN_POS && pos.x <= MAX_X && pos.y >= MIN_POS && pos.y <= MAX_Y
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
