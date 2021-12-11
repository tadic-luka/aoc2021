pub const ROWS: usize = 5;
pub const COLS: usize = ROWS;
pub const SIZE: usize = ROWS * COLS;

#[derive(Debug)]
pub struct Board {
    data: [usize; SIZE],
    marked: [bool; SIZE],
}

impl Board {
    pub fn new(data: [usize; SIZE]) -> Self {
        Self {
            data,
            marked: [false; SIZE],
        }
    }

    pub fn reset(&mut self) {
        for val in &mut self.marked {
            *val = false;
        }
    }

    fn is_win(&self) -> bool {
        for i in 0..ROWS {
            if self.marked[i * COLS..(i + 1) * COLS].iter().all(|&val| val) {
                return true;
            }
            if self.marked.iter().skip(i).step_by(COLS).all(|&val| val) {
                return true;
            }
        }
        false
    }

    fn mark_num(&mut self, num: usize) {
        for i in 0..SIZE {
            if self.data[i] == num {
                self.marked[i] = true;
            }
        }
    }

    fn sum_unmarked(&self) -> usize {
        self.marked
            .into_iter()
            .zip(self.data.into_iter())
            .filter(|&(marked, _)| !marked)
            .map(|(_, number)| number)
            .sum()
    }
}

pub fn solve_puzzle_one(numbers: &[usize], mut boards: Vec<Board>) -> usize {
    for &number in numbers {
        for board in &mut boards {
            board.mark_num(number);
            if board.is_win() {
                return board.sum_unmarked() * number;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let numbers = [
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let boards = vec![
            Board::new([
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19,
            ]),
            Board::new([
                3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16,
                12, 6,
            ]),
            Board::new([
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ]),
        ];

        assert_eq!(solve_puzzle_one(&numbers, boards), 4512);
    }

    #[test]
    fn board_win_test() {
        let mut board = Board::new([
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ]);
        board.mark_num(22);
        board.mark_num(13);
        board.mark_num(17);
        board.mark_num(11);
        board.mark_num(0);
        assert!(board.is_win());

        board.reset();
        board.mark_num(22);
        board.mark_num(8);
        board.mark_num(21);
        board.mark_num(6);
        board.mark_num(1);
        assert!(board.is_win());

        board.reset();
        board.mark_num(13);
        board.mark_num(17);
        board.mark_num(11);
        board.mark_num(0);
        board.mark_num(8);
        assert!(!board.is_win());
    }
}
