use std::io::{Error, ErrorKind, Result};

use aoc_2021::{get_file, get_input_contents, solution::day08::solve_puzzle_one};

fn main() -> Result<()> {
    let file = get_file()?;
    let input = get_input_contents(file)?;

    let input: Vec<_> = input
        .trim()
        .split('|')
        .enumerate()
        .map(|(i, val)| {
            val.trim().parse().map_err(|_| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid display provided at index  {}, {}", i, val),
                )
            })
        })
        .collect::<Result<_>>()?;

    let result = solve_puzzle_one(&input);
    println!("{}", result);
    Ok(())
}
