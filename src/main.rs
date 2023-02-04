use anyhow::anyhow;
use anyhow::Result;
use std::io;
mod lib;

fn main() -> Result<()> {
    println!("input numbers: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let numbers = parse_input(buffer)?;

    let solutions = lib::solve(numbers);
    solutions.iter().for_each(|exp| {
        println!("{}", exp);
    });

    Ok(())
}

fn parse_input(input: String) -> Result<[u8; 4]> {
    let nums_raw = input.split_whitespace().collect::<Vec<&str>>();
    if nums_raw.len() != 4 {
        return Err(anyhow!("please input 4 numbers as arguments"));
    }

    let a = nums_raw[0].parse::<u8>()?;
    let b = nums_raw[1].parse::<u8>()?;
    let c = nums_raw[2].parse::<u8>()?;
    let d = nums_raw[3].parse::<u8>()?;

    Ok([a, b, c, d])
}
