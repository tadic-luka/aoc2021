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

pub fn solve_puzzle_two(input: &[usize]) -> usize {
    if input.len() < 3 {
        return 0;
    }
    let start = input[0] + input[1] + input[2];
    input[1..].windows(3)
        .scan(start, |last_sum, new| {
            let new_sum = new[0] + new[1] + new[2];
            let res = usize::from(*last_sum < new_sum);
            *last_sum = new_sum;
            Some(res)
        }).sum()
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

    #[test]
    fn test_puzzle_2() {
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
        assert_eq!(solve_puzzle_two(&input), 5);
    }
}
