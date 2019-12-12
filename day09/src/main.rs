use std::collections::HashMap;
use std::collections::VecDeque;

enum ProgramResult {
    WaitForInputAt,
    Output(i64),
    Halted,
}

struct ProgramState {
    program: HashMap<u64, i64>,
    inputs: VecDeque<i64>,
    pc: u64,
    relative_base: i64,
}

impl ProgramState {
    fn get(&self, i: u64) -> i64 {
        *self.program.get(&i).unwrap_or(&0)
    }
    fn set(&mut self, i: u64, val: i64) {
        self.program.insert(i, val);
    }

    fn get_param_address(&self, p: u64) -> u64 {
        let pmode = self.get(self.pc) / (100 * 10i64.pow(p as u32)) % 10;
        match pmode {
            0 => self.get(self.pc + p + 1) as u64, // position mode
            1 => self.pc + p + 1,                  // value mode
            2 => (self.get(self.pc + p + 1) + self.relative_base) as u64, // relative mode
            _ => panic!("unknown pmode"),
        }
    }

    fn get_param(&self, p: u64) -> i64 {
        self.get(self.get_param_address(p))
    }

    fn eval_program(&mut self) -> ProgramResult {
        loop {
            let opcode = self.program[&self.pc] % 100;
            match opcode {
                1 => self.set(
                    self.get_param_address(2),
                    self.get_param(0) + self.get_param(1),
                ),
                2 => self.set(
                    self.get_param_address(2),
                    self.get_param(0) * self.get_param(1),
                ),
                3 => {
                    let val = match self.inputs.pop_front() {
                        Some(input) => input,
                        None => return ProgramResult::WaitForInputAt,
                    };
                    self.set(self.get_param_address(0), val);
                }
                4 => {
                    let output = self.get_param(0);
                    self.pc += 2;
                    return ProgramResult::Output(output);
                }
                5 => {
                    if self.get_param(0) != 0 {
                        self.pc = self.get_param(1) as u64;
                        continue;
                    }
                }
                6 => {
                    if self.get_param(0) == 0 {
                        self.pc = self.get_param(1) as u64;
                        continue;
                    }
                }
                7 => self.set(
                    self.get_param_address(2),
                    if self.get_param(0) < self.get_param(1) {
                        1
                    } else {
                        0
                    },
                ),
                8 => self.set(
                    self.get_param_address(2),
                    if self.get_param(0) == self.get_param(1) {
                        1
                    } else {
                        0
                    },
                ),
                9 => self.relative_base += self.get_param(0),
                99 => return ProgramResult::Halted,
                _ => panic!("Invalid opcode!"),
            };
            self.pc += match opcode {
                1 => 4,
                2 => 4,
                3 => 2,
                4 => 2,
                5 => 3,
                6 => 3,
                7 => 4,
                8 => 4,
                9 => 2,
                _ => panic!("Invalid opcode!"),
            };
        }
    }
}

fn parse_program(puzzle_input: &str) -> HashMap<u64, i64> {
    let mut program = HashMap::new();
    for (i, instr) in puzzle_input.split(',').enumerate() {
        program.insert(i as u64, instr.parse::<i64>().unwrap());
    }
    program
}

fn run_program(puzzle_input: &str, input: &[i64]) -> Vec<i64> {
    let mut program = ProgramState {
        program: parse_program(puzzle_input),
        inputs: input.into_iter().map(|i| *i).collect(),
        pc: 0,
        relative_base: 0,
    };

    let mut output = Vec::new();
    loop {
        match program.eval_program() {
            ProgramResult::Output(out) => output.push(out),
            ProgramResult::Halted => return output,
            _ => panic!("invalid program state"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples_day09_part1() {
        assert_eq!(
            super::run_program(
                "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
                &[] as &[i64]
            ),
            [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
        assert_eq!(
            super::run_program("1102,34915192,34915192,7,4,7,99,0", &[] as &[i64]),
            [1219070632396864]
        );
        assert_eq!(
            super::run_program("104,1125899906842624,99", &[] as &[i64]),
            [1125899906842624]
        );
    }

    #[test]
    fn samples_day09_part2() {}
}

fn main() {
    let puzzle_input = include_str!("input.txt");
    println!("{:?}", run_program(puzzle_input, &[2]))
}
