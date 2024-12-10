use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug, Clone)]
struct Block {
    id: usize,
    free: bool,
}

fn part_one(file: &File) -> usize {
    let mut reader: BufReader<&File> = BufReader::new(file);
    let mut fs = String::new();
    let mut blocks: Vec<Block> = Vec::new();
    reader.read_line(&mut fs).unwrap();
    fs.chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
        .chunks(2)
        .enumerate()
        .for_each(|(id, chunk)| {
            for _ in 0..chunk[0] {
                blocks.push(Block{id, free: false})
            }
            if chunk.len() > 1 {
                for _ in 0..chunk[1] {
                    blocks.push(Block{id, free: true});
                }
            }
        });
    let mut b = blocks.len() - 1;
    let mut f = next_free_index(&blocks, 0);
    while b > f {
        blocks.swap(b, f);
        b -= 1;
        while blocks[b].free {
            b -= 1;
        }
        f = next_free_index(&blocks, f);
    }

    blocks.iter().enumerate().filter(|(_, block)| !block.free).map(|(i, block)| i * block.id).sum() 
}

fn next_free_index(blocks: &Vec<Block>, mut index: usize) -> usize {
    index += 1;
    for i in index..blocks.len() {
        if blocks[i].free {
            return i;
        }
    }
    blocks.len()
}

fn part_two(file: &File) -> usize {
    let reader: BufReader<&File> = BufReader::new(file);
    0
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
