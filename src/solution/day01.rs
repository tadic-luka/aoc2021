pub struct Day01;

pub fn solve_puzzle_one(input: &[usize]) -> usize {
    input.windows(2)
        .map(|input| {
            match input {
                &[prev, curr] => {
                    usize::from(prev < curr)
                }
                _ => 0
            }
        })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        let input = [
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263
        ];
        assert_eq!(solve_puzzle_one(&input), 7);
    }
}
