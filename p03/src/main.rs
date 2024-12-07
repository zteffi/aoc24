use std::fs;
use std::io::{self};
use regex::Regex;

fn solve_one(file_path : &str) -> i64
{
    let contents: String = fs::read_to_string(file_path).expect("REASON");
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut res:i64 = 0;
    for (_, [a,b]) in re.captures_iter(&contents).map(|c| c.extract()) {
        res += a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
    }
    return res;
}



// Example usage
fn main() -> io::Result<()>
{
    println!("R1:{}", solve_one("input.txt"));

    Ok(())
}
