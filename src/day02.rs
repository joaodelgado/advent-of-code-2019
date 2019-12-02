use super::Day;

struct Computer {
    instructions: Vec<usize>,
    pc: usize,
}

impl Computer {
    fn new(instructions: Vec<usize>) -> Computer {
        Computer {
            pc: 0,
            instructions,
        }
    }

    fn init(&mut self, noun: usize, verb: usize) {
        self.instructions[1] = noun;
        self.instructions[2] = verb;
    }

    fn run(&mut self) {
        loop {
            match self.next() {
                1 => self.add(),
                2 => self.mul(),
                99 => return,
                _ => unreachable!(),
            }
        }
    }

    fn add(&mut self) {
        let a = self.next();
        let a = self.instructions[a];

        let b = self.next();
        let b = self.instructions[b];

        let dest = self.next();
        self.instructions[dest] = a + b;
    }

    fn mul(&mut self) {
        let a = self.next();
        let a = self.instructions[a];

        let b = self.next();
        let b = self.instructions[b];

        let dest = self.next();
        self.instructions[dest] = a * b;
    }

    fn next(&mut self) -> usize {
        let i = self.pc as usize;
        self.pc += 1;

        self.instructions[i]
    }
}

pub struct Day02;

impl Day<usize, usize> for Day02 {
    fn run1(input: &str) -> usize {
        let insts = input.split(",").filter_map(|m| m.parse().ok()).collect();

        let mut computer = Computer::new(insts);
        computer.init(12, 02);
        computer.run();

        return computer.instructions[0];
    }

    fn run2(input: &str) -> usize {
        let insts = input
            .split(",")
            .filter_map(|m| m.parse().ok())
            .collect::<Vec<_>>();

        for noun in 0..100 {
            for verb in 0..100 {
                let mut computer = Computer::new(insts.clone());
                computer.init(noun, verb);
                computer.run();

                if computer.instructions[0] == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1() {
        let mut computer = Computer::new(vec![1, 0, 0, 0, 99]);
        computer.run();
        assert_eq!(computer.instructions, vec![2, 0, 0, 0, 99]);

        let mut computer = Computer::new(vec![2, 3, 0, 3, 99]);
        computer.run();
        assert_eq!(computer.instructions, vec![2, 3, 0, 6, 99]);

        let mut computer = Computer::new(vec![2, 4, 4, 5, 99, 0]);
        computer.run();
        assert_eq!(computer.instructions, vec![2, 4, 4, 5, 99, 9801]);

        let mut computer = Computer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        computer.run();
        assert_eq!(computer.instructions, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);

        assert_eq!(Day02::run1(include_str!("../data/day02")), 5534943);
    }
}
