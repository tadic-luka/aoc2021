fn move_to_pos_cost(positions: &[usize], target: usize) -> usize {
    positions.iter().copied()
        .map(|pos| if pos < target {
            target - pos
        } else {
            pos - target
        })
        .sum()
}


pub fn solve_puzzle_one(positions: &[usize]) -> usize {
    let max = positions.iter().copied().max().unwrap_or_default();
    (0..max).fold(usize::MAX, |min, pos| {
            std::cmp::min(min, move_to_pos_cost(positions, pos))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = [16,1,2,0,4,2,7,1,2,14];
        assert_eq!(solve_puzzle_one(&input), 37);
    }
}
