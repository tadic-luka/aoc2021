struct Emulator {
    data: [usize; 9],
}

impl Emulator {
    fn new(start: &[usize]) -> Self {
        let mut data  = [0; 9];
        for &input in start {
            data[input] += 1;
        }
        Self {
            data
        }
    }
    fn next_day(&mut self) {
        let zero = self.data[0];
        self.data[0] = self.data[1];
        self.data[1] = self.data[2];
        self.data[2] = self.data[3];
        self.data[3] = self.data[4];
        self.data[4] = self.data[5];
        self.data[5] = self.data[6];
        self.data[6] = self.data[7] + zero;
        self.data[7] = self.data[8];
        self.data[8] = zero;
    }

    fn emulate(&mut self, days: usize) {
        for _ in 0..days {
            self.next_day();
        }
    }

    fn current_size(&self) -> usize {
        self.data.into_iter().sum()
    }
}

pub fn solve_puzzle_one(input: &[usize], days: usize) -> usize {
    let mut emulator = Emulator::new(input);
    emulator.emulate(days);
    emulator.current_size()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let mut input = [3, 4, 3, 1, 2];
        assert_eq!(solve_puzzle_one(&input, 80), 5934);
        assert_eq!(solve_puzzle_one(&input, 256), 26984457539);
    }
}
