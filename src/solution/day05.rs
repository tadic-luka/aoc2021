use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct Board {
    data: HashMap<Point, usize>,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }
    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    fn min_x(&self) -> usize {
        std::cmp::min(self.start.x, self.end.x)
    }
    fn min_y(&self) -> usize {
        std::cmp::min(self.start.y, self.end.y)
    }
    fn max_x(&self) -> usize {
        std::cmp::max(self.start.x, self.end.x)
    }
    fn max_y(&self) -> usize {
        std::cmp::max(self.start.y, self.end.y)
    }
}

impl Board {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn add_point(&mut self, point: Point) {
        *self.data.entry(point).or_default() += 1;
    }

    fn add_horiz_or_vert_line(&mut self, line: Line) {
        if !line.is_vertical() && !line.is_horizontal() {
            return;
        }
        if line.is_horizontal() {
            let x = line.min_x();
            let min_y = line.min_y();
            let max_y = line.max_y();
            for y in min_y..max_y + 1 {
                self.add_point(Point::new(x, y));
            }
        } else {
            let y = line.min_y();
            let min_x = line.min_x();
            let max_x = line.max_x();
            for x in min_x..max_x + 1 {
                self.add_point(Point::new(x, y));
            }
        }
    }

    fn add_line(&mut self, line: Line) {
        if line.is_horizontal() || line.is_vertical() {
            self.add_horiz_or_vert_line(line);
            return;
        }
        let mut x = line.start.x;
        let mut y = line.start.y;
        loop {
            self.add_point(Point::new(x, y));
            if x == line.end.x {
                break;
            }
            if y == line.end.y {
                break;
            }
            if x < line.end.x {
                x += 1;
            } else {
                x -= 1;
            }
            if y < line.end.y {
                y += 1;
            } else {
                y -= 1;
            }
        }
    }
    fn at_least_two_overlaps(&self) -> usize {
        self.data.iter().filter(|(_, &val)| val >= 2).count()
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.split(',');
        let x = data.next().ok_or("Missing point x".to_string())?;
        let y = data.next().ok_or("Missing point y".to_string())?;

        let x = x.parse().map_err(|_| "Invalid integer give".to_string())?;
        let y = y.parse().map_err(|_| "Invalid integer give".to_string())?;

        Ok(Self { x, y })
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let start = parts
            .next()
            .ok_or("Invalid line given, missing start".to_string())?;
        parts
            .next()
            .ok_or("Invalid line given, missing -> part".to_string())?;
        let end = parts
            .next()
            .ok_or("Invalid line given, missing end".to_string())?;

        Ok(Self::new(start.parse()?, end.parse()?))
    }
}

pub fn solve_puzzle_one(input: &[Line]) -> usize {
    let mut board = Board::new();
    for &line in input
        .into_iter()
        .filter(|line| line.is_vertical() || line.is_horizontal())
    {
        board.add_line(line);
    }
    board.at_least_two_overlaps()
}

pub fn solve_puzzle_two(input: &[Line]) -> usize {
    let mut board = Board::new();
    for &line in input {
        board.add_line(line);
    }
    board.at_least_two_overlaps()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = [
            Line::new(Point::new(0, 9), Point::new(5, 9)),
            Line::new(Point::new(8, 0), Point::new(0, 8)),
            Line::new(Point::new(9, 4), Point::new(3, 4)),
            Line::new(Point::new(2, 2), Point::new(2, 1)),
            Line::new(Point::new(7, 0), Point::new(7, 4)),
            Line::new(Point::new(6, 4), Point::new(2, 0)),
            Line::new(Point::new(0, 9), Point::new(2, 9)),
            Line::new(Point::new(3, 4), Point::new(1, 4)),
            Line::new(Point::new(0, 0), Point::new(8, 8)),
            Line::new(Point::new(5, 5), Point::new(8, 2)),
        ];
        assert_eq!(solve_puzzle_one(&input), 5);
    }

    #[test]
    fn part_2_test() {
        let input = [
            Line::new(Point::new(0, 9), Point::new(5, 9)),
            Line::new(Point::new(8, 0), Point::new(0, 8)),
            Line::new(Point::new(9, 4), Point::new(3, 4)),
            Line::new(Point::new(2, 2), Point::new(2, 1)),
            Line::new(Point::new(7, 0), Point::new(7, 4)),
            Line::new(Point::new(6, 4), Point::new(2, 0)),
            Line::new(Point::new(0, 9), Point::new(2, 9)),
            Line::new(Point::new(3, 4), Point::new(1, 4)),
            Line::new(Point::new(0, 0), Point::new(8, 8)),
            Line::new(Point::new(5, 5), Point::new(8, 2)),
        ];
        assert_eq!(solve_puzzle_two(&input), 12);
    }
}
