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
                .any(|i| eval(operands, i) == *right_answer)
        })
        .map(|(ans, _)| ans)
        .sum::<i64>()
}

fn eval(operands: &Vec<i64>, operators: i32) -> i64 {
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

fn main() -> io::Result<()> {
    let mut file = File::open("src/input.txt")?;
    let p1 = part_one(&file);
    println!("Part 1: {}", p1);
    file.rewind()?;
    // let p2 = part_two(&file);
    // println!("Part 2: {}", p2);
    Ok(())
}
