use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Number(pub Vec<bool>);

impl Number {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn to_bit_str(&self) -> String {
        self.0
            .iter()
            .map(|val| match val {
                true => '1',
                false => '0',
            })
            .collect()
    }
}

impl FromStr for Number {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Result<Vec<bool>, Self::Err> = s
            .chars()
            .map(|ch| match ch {
                '1' => Ok(true),
                '0' => Ok(false),
                _ => Err("Invalid input, must be 0 or 1".to_string()),
            })
            .collect();
        vec.map(Number)
    }
}

pub fn solve_puzzle_one(input: &[Number]) -> usize {
    if input.is_empty() {
        return 0;
    }
    let len = input[0].len();
    let mut gamma = String::with_capacity(len);
    let mut epsilon = String::with_capacity(len);

    for i in 0..len {
        let ones = (0..input.len())
            .map(|j| input[j].0[i])
            .filter(|&val| val)
            .count();
        let (char_gamma, char_epsilon) = match ones > input.len() / 2 {
            true => ('1', '0'),
            false => ('0', '1'),
        };
        gamma.push(char_gamma);
        epsilon.push(char_epsilon);
    }
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();
    gamma * epsilon
}

fn reduce(input: &[Number], reverse: bool) -> &Number {
    let len = input[0].len();
    let mut remaining: Vec<usize> = (0..input.len()).collect();
    for i in 0..len {
        let half = (remaining.len() + 1) / 2;
        let ones = (0..remaining.len())
            .map(|j| input[remaining[j]].0[i])
            .filter(|&val| val)
            .count() >= half;
        remaining.retain(|&index| {
            let val = input[index].0[i];
            (reverse && ((!ones && val) || (ones && !val)))
                || (!reverse && ((ones && val) || (!ones && !val)))
        });
        if remaining.len() == 1 {
            return &input[remaining[0]];
        }
    }
    unreachable!()
}

pub fn solve_puzzle_two(input: &[Number]) -> usize {
    if input.is_empty() {
        return 0;
    }

    let oxy = reduce(input, false);
    let co = reduce(input, true);

    let co: String = co.to_bit_str();
    let oxy: String = oxy.to_bit_str();

    let co = usize::from_str_radix(&co, 2).unwrap();
    let oxy = usize::from_str_radix(&oxy, 2).unwrap();
    co * oxy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = [
            Number(vec![false, false, true, false, false]), // 00100
            Number(vec![true, true, true, true, false]),    // 11110
            Number(vec![true, false, true, true, false]),   // 10110
            Number(vec![true, false, true, true, true]),    // 10111
            Number(vec![true, false, true, false, true]),   // 10101
            Number(vec![false, true, true, true, true]),    // 01111
            Number(vec![false, false, true, true, true]),   // 00111
            Number(vec![true, true, true, false, false]),   // 11100
            Number(vec![true, false, false, false, false]), // 10000
            Number(vec![true, true, false, false, true]),   // 11001
            Number(vec![false, false, false, true, false]), // 00010
            Number(vec![false, true, false, true, false]),  // 01010
        ];

        assert_eq!(solve_puzzle_one(&input), 198);
    }

    #[test]
    fn part_2_test() {
        let input = [
            Number(vec![false, false, true, false, false]), // 00100
            Number(vec![true, true, true, true, false]),    // 11110
            Number(vec![true, false, true, true, false]),   // 10110
            Number(vec![true, false, true, true, true]),    // 10111
            Number(vec![true, false, true, false, true]),   // 10101
            Number(vec![false, true, true, true, true]),    // 01111
            Number(vec![false, false, true, true, true]),   // 00111
            Number(vec![true, true, true, false, false]),   // 11100
            Number(vec![true, false, false, false, false]), // 10000
            Number(vec![true, true, false, false, true]),   // 11001
            Number(vec![false, false, false, true, false]), // 00010
            Number(vec![false, true, false, true, false]),  // 01010
        ];

        assert_eq!(solve_puzzle_two(&input), 230);
    }
}
