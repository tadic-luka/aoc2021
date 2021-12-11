use aoc_2021::{
    get_file, get_input_contents, parse_day4_boards, solution::day04::solve_puzzle_one,
};

fn main() -> std::io::Result<()> {
    let file = get_file()?;
    let input = get_input_contents(file)?;

    let (numbers, boards) = parse_day4_boards(input)?;

    let result = solve_puzzle_one(&numbers, boards);
    println!("{}", result);
    Ok(())
}
