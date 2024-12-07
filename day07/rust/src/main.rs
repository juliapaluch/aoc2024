use std::{
    fs::File,
    io::{self, BufRead, BufReader, Seek},
};


fn part_one(file: &File) -> i64 {
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.split(':').map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|x| {
            (
                x[0].clone().parse::<i64>().unwrap(),
                x[1].split_whitespace()
                    .map(|s| s.to_string().parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(right_answer, operands)| {
            (0..2_i32.pow((operands.len() - 1).try_into().unwrap()))
                .any(|i| eval_part_one(operands, i) == *right_answer)
        })
        .map(|(ans, _)| ans)
        .sum::<i64>()
}

fn eval_part_one(operands: &Vec<i64>, operators: i32) -> i64 {
    let mut answer = operands[0];
    for l in 0..operands.len() - 1 {
        if operators >> l & 1 == 0 {
            answer += operands[l + 1];
        } else {
            answer *= operands[l + 1];
        }
    }
    answer
}

fn get_base_three(mut operators: i32, desired_size: usize) -> Vec<u8> {
    let mut base_three: Vec<u8> = Vec::new();
    while operators / 3 > 0 {
        base_three.push((operators % 3).try_into().unwrap());
        operators = operators / 3;
    }
    base_three.push((operators % 3).try_into().unwrap());
    (0..desired_size-base_three.len()).for_each(|_| base_three.push(0));
    base_three.reverse();
    base_three
}

fn part_two(file: &File) -> u64 {
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.split(':').map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|x| {
            (
                x[0].clone().parse::<u64>().unwrap(),
                x[1].split_whitespace()
                    .map(|s| s.to_string().parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(right_answer, operands)| {
            (0..3_i32.pow((operands.len() - 1).try_into().unwrap()))
                .any(|i| eval_part_two(operands, i) == *right_answer)
        })
        .map(|(ans, _)| ans)
        .sum::<u64>()
}

fn eval_part_two(operands: &Vec<u64>, operators: i32) -> u64 {
    let b3_ops = get_base_three(operators, operands.len()-1);
    let mut answer = operands[0];
    for l in 0..operands.len() - 1 {
        if b3_ops[l] == 0 {
            answer += operands[l + 1];
        } else if b3_ops[l] == 1{
            answer *= operands[l + 1];
        } else if b3_ops[l] == 2 {
            answer = format!("{}{}",answer, operands[l + 1]).parse::<u64>().unwrap();
        }
    }
    answer
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
