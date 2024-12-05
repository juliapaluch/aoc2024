use std::{
    collections::{hash_map::Entry, HashMap},
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

fn part_one(file: &File) -> u32 {
    let reader = BufReader::new(file);
    let (rules, updates): (Vec<_>, Vec<_>) = reader
        .lines()
        .map(|x| x.unwrap())
        .partition(|x| x.contains('|'));

    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();
    rules
        .iter()
        .map(|x| x.split('|').collect())
        .for_each(|x: Vec<_>| {
            let k = x[1].parse::<u32>().unwrap();
            let v = x[0].parse::<u32>().unwrap();
            match rules_map.entry(k) {
                Entry::Vacant(a) => {
                    a.insert(vec![v]);
                }
                Entry::Occupied(mut a) => {
                    a.get_mut().push(v);
                }
            }
        });

    updates
        .iter()
        .filter(|v| v.len() > 0)
        .map(|x| {
            x.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|x: &Vec<u32>| {
            x.iter().enumerate().all(|(i, c)| {
                if rules_map.contains_key(c) {
                    rules_map
                        .get(c)
                        .unwrap()
                        .iter()
                        .filter(|p| x.contains(&p))
                        .all(|p| (&x[0..i]).contains(&p))
                } else {
                    true
                }
            })
        })
        .map(|x| x[x.len() / 2])
        .sum::<u32>()
}

fn part_two(file: &File) -> u32 {
    let reader = BufReader::new(file);
    let (rules, updates): (Vec<_>, Vec<_>) = reader
        .lines()
        .map(|x| x.unwrap())
        .partition(|x| x.contains('|'));

    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();
    rules
        .iter()
        .map(|x| x.split('|').collect())
        .for_each(|x: Vec<_>| {
            let k = x[1].parse::<u32>().unwrap();
            let v = x[0].parse::<u32>().unwrap();
            match rules_map.entry(k) {
                Entry::Vacant(a) => {
                    a.insert(vec![v]);
                }
                Entry::Occupied(mut a) => {
                    a.get_mut().push(v);
                }
            }
        });

    updates
        .iter()
        .filter(|v| v.len() > 0)
        .map(|x| {
            x.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|x: Vec<u32>| {
            let mut new_x = x.clone();
            for _ in 0..x.len() {
                let temp_x = new_x.clone();
                temp_x.iter().enumerate().for_each(|(i, c)| {
                    if rules_map.contains_key(c) {
                        rules_map
                            .get(c)
                            .unwrap()
                            .iter()
                            .filter(|p| x.contains(&p))
                            .for_each(|p| {
                                if !(&new_x[0..=i]).contains(&p) {
                                    let element =
                                        new_x.remove(new_x.iter().position(|e| e == p).unwrap());
                                    new_x.insert(i, element);
                                }
                            })
                    }
                });
            }
            new_x
        })
        .map(|x| x[x.len() / 2])
        .sum::<u32>()
}

fn main() -> io::Result<()> {
    let mut file = File::open("src/input.txt")?;
    let p1 = part_one(&file);
    println!("Part 1: {}", p1);
    file.rewind()?;
    // subtract part 1 from part 2 because we loop through all of them and I don't know how to
    // filter them out and also mutate the incorrect updates without doing the validation twice lol
    let p2 = part_two(&file);
    println!("Part 2: {}", p2 - p1);
    Ok(())
}
