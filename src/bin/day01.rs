use std::io;

use aoc_2021::{get_file, get_input_contents, solution::day01::solve_puzzle_one};

fn main() -> std::io::Result<()> {
    let file = get_file()?;
    let input = get_input_contents(file)?;

    let input = input.lines()
        .map(|line| line.trim().parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

    let result = solve_puzzle_one(&input);
    println!("{}", result);
    Ok(())
}

