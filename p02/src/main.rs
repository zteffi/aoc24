use std::fs::File;
use std::io::{self, BufRead};
use std::mem;

fn parse_file(file_path: &str) -> io::Result<Vec<Vec<i32>>>
{
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let result: Vec<Vec<i32>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect();

    Ok(result)
}

fn solve_one(table: &Vec<Vec<i32>>) ->i32
{
    let mut res:i32 = 0;
    for line in table
    {
        let inc = if line[0] < line[1] {1} else {-1};
        let mut prev = line[0];
        for i in 1..line.len()
        {
            let mut big = prev + 3*inc;
            let mut small = prev + inc;
            if inc < 0
            {
                mem::swap(&mut big, &mut small)
            }

            if small > line[i] || line[i] > big
            {
                res -= 1;
                break;
            }
            prev = line[i];
        }
        res += 1;
    }
    return res;
}

fn solve_two(table: &Vec<Vec<i32>>) ->i32
{
    let mut res:i32 = 0;
    for line in table
    {
        let inc = if line[0] < line[1] {1} else {-1};
        let mut prev = 0;
        let mut tolerance = 1;
        for i in 1..line.len()
        {
            let mut big = line[prev] + 3*inc;
            let mut small = line[prev] + inc;
            if inc < 0
            {
                mem::swap(&mut big, &mut small)
            }

            if small > line[i] || line[i] > big
            {
                tolerance -= 1;
                if tolerance == 0
                {
                    continue;
                }
                res -= 1;
                break;
            }
            prev = i;
        }
        res += 1;
    }
    return res;
}

// Example usage
fn main() -> io::Result<()> {
    let file_path = "input.txt";
    let input = parse_file(file_path)?;

    println!("R1:{}", solve_one(&input));
    println!("R1:{}", solve_two(&input));

    Ok(())
}
