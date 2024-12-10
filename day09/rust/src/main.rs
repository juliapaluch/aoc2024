use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};

#[derive(Debug, Copy, Clone)]
struct Block {
    id: usize,
    free: bool,
}
#[derive(Debug, Copy, Clone)]
struct FileBlock {
    id: usize,
    size: u8,
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
                blocks.push(Block { id, free: false })
            }
            if chunk.len() > 1 {
                for _ in 0..chunk[1] {
                    blocks.push(Block { id, free: true });
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

    blocks
        .iter()
        .enumerate()
        .filter(|(_, block)| !block.free)
        .map(|(i, block)| i * block.id)
        .sum()
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
    let mut reader: BufReader<&File> = BufReader::new(file);
    let mut fs = String::new();
    let mut blocks: Vec<FileBlock> = Vec::new();
    reader.read_line(&mut fs).unwrap();
    fs.chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
        .chunks(2)
        .enumerate()
        .for_each(|(id, chunk)| {
            blocks.push(FileBlock {
                id,
                size: chunk[0],
                free: false,
            });
            if chunk.len() > 1 {
                blocks.push(FileBlock {
                    id,
                    size: chunk[1],
                    free: true,
                });
            }
        });

    let mut block_to_move = (blocks.len() - 1) as i64;
    while block_to_move >= 0 {
        while blocks[block_to_move as usize].free {
            block_to_move -= 1;
        }
        if let Some(free_block) = blocks[0..block_to_move as usize]
            .iter()
            .position(|fb| fb.free && fb.size >= blocks[block_to_move as usize].size)
        {
            blocks[free_block].size -= blocks[block_to_move as usize].size;
            let moved = blocks[block_to_move as usize];
            blocks[block_to_move as usize].free = true;
            blocks.insert(free_block, moved);
        } else {
            block_to_move -= 1;
        }
    }

    let mut sum = 0;
    let mut position = 0;
    blocks.iter().for_each(|b| {
        for _ in 0..b.size {
            if !b.free {
                sum += b.id * position
            }
            position += 1;
        }
    });
    sum
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
