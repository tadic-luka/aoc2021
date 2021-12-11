use std::io::{Error, ErrorKind};

use aoc_2021::{get_file, get_input_contents, get_lines, solution::day03::{Number, solve_puzzle_two}};

fn main() -> std::io::Result<()> {
    let file = get_file()?;
    let input = get_input_contents(file)?;

    let input = get_lines(input)?;
    let all_same_length = input.windows(2).all(|win: &[Number]| match win {
        [numa, numb] => numa.len() == numb.len(),
        [_] => true,
        _ => unreachable!(),
    });
    if !all_same_length {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Not all binary numbers have the same length in the file".to_string(),
        ));
    }

    let result = solve_puzzle_two(&input);
    println!("{}", result);
    Ok(())
}
