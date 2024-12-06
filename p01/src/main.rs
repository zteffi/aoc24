use std::fs::File;
use core::iter::zip;
use std::io::{self, BufRead};

fn parse_file(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)>
{
    let mut left = Vec::new();
    let mut right = Vec::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if numbers.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
                left.push(num1);
                right.push(num2);
            }
        }
    }

    Ok((left, right))
}

fn solve_one(left: &mut Vec<i32>, right: &mut Vec<i32>) ->i32
{
    left.sort();
    right.sort();
    let mut res = 0;
    let ileft = left.iter();
    let iright = right.iter();
    for (l,r) in zip(ileft, iright) {
        res += (l-r).abs();
    }
    return res;
}

fn solve_two(left: Box<Vec<i32>>, right: Box<Vec<i32>>) ->i32
{
    let mut res = 0;
    let ileft = left.iter();
    let mut idx = 0;
    for l in ileft
    {
        while right[idx] <= *l
            {
                if right[idx] == *l
                {
                    res += *l;
                }
                if right.len() == idx+1 {
                    return res;
                }
                idx+= 1;
            }
    }
    return res;
}



// Example usage
fn main() -> io::Result<()> {
    let file_path = "input.txt"; // Replace with your file path
    let (mut left, mut right) = parse_file(file_path)?;

    println!("R1:{}", solve_one(&mut left, &mut right));
    println!("R2:{}", solve_two(Box::new(left), Box::new(right)));

    Ok(())
}
