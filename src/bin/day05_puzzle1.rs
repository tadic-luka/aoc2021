use aoc_2021::{get_file, get_input_contents, get_lines, solution::day05::solve_puzzle_one};

fn main() -> std::io::Result<()> {
    let file = get_file()?;
    let input = get_input_contents(file)?;

    let input = get_lines(input)?;

    let result = solve_puzzle_one(&input);
    println!("{}", result);
    Ok(())
    
}
