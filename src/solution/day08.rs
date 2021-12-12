use std::str::FromStr;

#[derive(Debug, Clone)]
struct Digit {
    segments: Vec<Segment>,
}

#[derive(Debug, Clone)]
pub struct Display {
    digits: Vec<Digit>
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Digit {
    fn new(segments: Vec<Segment>) -> Self {
        Self { segments }
    }
}

impl Display {
    fn new(digits: Vec<Digit>) -> Self {
        Self { digits }
    }
}

impl TryFrom<char> for Segment {
    type Error = String;

    fn try_from(val: char) -> Result<Self, Self::Error> {
        match val {
            'a' => Ok(Self::A),
            'b' => Ok(Self::A),
            'c' => Ok(Self::A),
            'd' => Ok(Self::A),
            'e' => Ok(Self::A),
            'f' => Ok(Self::A),
            'g' => Ok(Self::A),
            char => Err(format!("Invalid segment name: {}", char)),
        }
    }
}

impl FromStr for Digit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = s
            .trim()
            .chars()
            .map(|v| Segment::try_from(v))
            .collect::<Result<Vec<Segment>, _>>()?;
        Ok(Self::new(segments))
    }
}

impl FromStr for Display {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .split_whitespace()
            .map(|digit| digit.parse::<Digit>())
            .collect::<Result<Vec<Digit>, _>>()?;

        Ok(Self { digits })
    }
}

pub fn solve_puzzle_one(input: &[Display]) -> usize {
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let displays: Vec<Display> = input
            .split("|")
            .enumerate()
            .map(|(index, val)| {
                val.trim()
                    .parse()
                    .map_err(|err| format!("Invalid display at index {}: {}", index, err))
            })
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(solve_puzzle_one(&displays), 26);
    }
}
