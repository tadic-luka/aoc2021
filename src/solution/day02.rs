use std::{num::ParseIntError, str::FromStr};

pub type NumType = u128;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Forward(NumType),
    Down(NumType),
    Up(NumType),
}

#[derive(Debug, Clone, Copy)]
struct Position {
    horizontal: NumType,
    depth: NumType,
}
impl Position  {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }

    fn apply(self, command: Command) -> Self {
        match command {
            Command::Forward(num) => Self {
                horizontal: self.horizontal + num,
                ..self
            },
            Command::Down(num) => Self {
                depth: self.depth + num,
                ..self
            },
            Command::Up(num) => Self {
                depth: self.depth - num,
                ..self
            },
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        match (iter.next(), iter.next()) {
            (Some(word), Some(num)) => {
                let num = num.parse().map_err(|err: ParseIntError| err.to_string())?;
                match word {
                    "forward" => Ok(Self::Forward(num)),
                    "down" => Ok(Self::Down(num)),
                    "up" => Ok(Self::Up(num)),
                    _ => Err("Word needs to be one of: forward|down|up".to_string()),
                }
            }
            _ => Err("Need to provide input as: <word> <number>".to_string()),
        }
    }
}

pub fn solve_puzzle_one(input: &[Command]) -> NumType {
    let pos = input
        .into_iter()
        .fold(Position::new(), |pos, &input| pos.apply(input));
    pos.horizontal * pos.depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = [
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        assert_eq!(solve_puzzle_one(&input), 150);
    }
}
