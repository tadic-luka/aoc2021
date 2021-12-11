fn get_distances<'input>(
    positions: &'input [usize],
    target: usize,
) -> impl Iterator<Item = usize> + 'input {
    positions.iter().copied().map(move |pos| {
        if pos < target {
            target - pos
        } else {
            pos - target
        }
    })
}
fn move_to_pos_cost_linear(positions: &[usize], target: usize) -> usize {
    get_distances(positions, target).sum()
}

fn move_to_pos_cost_non_linear(positions: &[usize], target: usize) -> usize {
    get_distances(positions, target)
        .map(|dist| dist * (dist + 1) / 2)
        .sum()
}

pub fn solve_puzzle_one(positions: &[usize]) -> usize {
    let max = positions.iter().copied().max().unwrap_or_default();
    (0..max)
        .map(|pos| move_to_pos_cost_linear(positions, pos))
        .fold(usize::MAX, std::cmp::min)
}

pub fn solve_puzzle_two(positions: &[usize]) -> usize {
    let max = positions.iter().copied().max().unwrap_or_default();
    (0..max)
        .map(|pos| move_to_pos_cost_non_linear(positions, pos))
        .fold(usize::MAX, std::cmp::min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(solve_puzzle_one(&input), 37);
    }

    #[test]
    fn part_2_test() {
        let input = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(solve_puzzle_two(&input), 168);
    }
}
