use std::collections::VecDeque;

enum ProgramResult {
    WaitForInputAt,
    Output(i32),
    Halted,
}

struct ProgramState {
    program: Vec<i32>,
    inputs: VecDeque<i32>,
    pc: usize,
}

impl ProgramState {
    fn get_param(&self, p: usize) -> i32 {
        let pmode = self.program[self.pc] / (100 * (10 as i32).pow(p as u32)) % 10;
        assert!(pmode == 0 || pmode == 1);
        if pmode == 0 {
            self.program[self.program[self.pc + p + 1] as usize]
        } else {
            self.program[self.pc + p + 1]
        }
    }
}

fn parse_program(puzzle_input: &str) -> Vec<i32> {
    puzzle_input
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn eval_program(state: &mut ProgramState) -> ProgramResult {
    loop {
        let opcode = state.program[state.pc] % 100;
        match opcode {
            1 => {
                let tidx = state.program[state.pc + 3] as usize;
                state.program[tidx] = state.get_param(0) + state.get_param(1)
            }
            2 => {
                let tidx = state.program[state.pc + 3] as usize;
                state.program[tidx] = state.get_param(0) * state.get_param(1)
            }
            3 => {
                let tidx = state.program[state.pc + 1] as usize;
                state.program[tidx] = match state.inputs.pop_front() {
                    Some(input) => input,
                    None => return ProgramResult::WaitForInputAt,
                };
            }
            4 => {
                let output = state.get_param(0);
                state.pc += 2;
                return ProgramResult::Output(output);
            }
            5 => {
                if state.get_param(0) != 0 {
                    state.pc = state.get_param(1) as usize;
                    continue;
                }
            }
            6 => {
                if state.get_param(0) == 0 {
                    state.pc = state.get_param(1) as usize;
                    continue;
                }
            }
            7 => {
                let tidx = state.program[state.pc + 3] as usize;
                state.program[tidx] = if state.get_param(0) < state.get_param(1) {
                    1
                } else {
                    0
                }
            }
            8 => {
                let tidx = state.program[state.pc + 3] as usize;
                state.program[tidx] = if state.get_param(0) == state.get_param(1) {
                    1
                } else {
                    0
                }
            }
            99 => return ProgramResult::Halted,
            _ => panic!("Invalid opcode!"),
        };

        state.pc += match opcode {
            1 => 4,
            2 => 4,
            3 => 2,
            4 => 2,
            5 => 3,
            6 => 3,
            7 => 4,
            8 => 4,
            _ => panic!("Invalid opcode!"),
        };
    }
}

fn run_amplifiers<'a>(program: &Vec<i32>, configs: impl Iterator<Item = &'a i32>) -> i32 {
    let mut signal = 0;
    for config in configs {
        let mut state = ProgramState {
            program: program.clone(),
            pc: 0,
            inputs: vec![*config, signal].into_iter().collect(),
        };
        signal = match eval_program(&mut state) {
            ProgramResult::Output(output) => output,
            _ => panic!("expect output on every program"),
        }
    }
    signal
}

fn run_amplifier_loop<'a>(program: &Vec<i32>, configs: impl Iterator<Item = &'a i32>) -> i32 {
    let mut signal = Some(0);
    let mut final_signal = -1;

    let mut amplifiers: Vec<ProgramState> = configs
        .map(|c| ProgramState {
            program: program.clone(),
            pc: 0,
            inputs: vec![*c].into_iter().collect(),
        })
        .collect();

    loop {
        for mut amplifier in amplifiers.iter_mut() {
            if let Some(s) = signal {
                amplifier.inputs.push_back(s);
            }
            signal = match eval_program(&mut amplifier) {
                ProgramResult::WaitForInputAt => None,
                ProgramResult::Output(out) => Some(out),
                ProgramResult::Halted => return final_signal,
            }
        }
        final_signal = signal.unwrap();
    }
}

fn find_max_amplifier_config(program: &Vec<i32>) -> i32 {
    let base_config = [0, 1, 2, 3, 4];
    let mut best = 0;
    for permutation in permute::permutations_of(&base_config) {
        let cur = run_amplifiers(&program, permutation);
        best = std::cmp::max(best, cur);
    }
    best
}

fn find_max_amplifier_loop(program: &Vec<i32>) -> i32 {
    let base_config = [9, 8, 7, 6, 5];
    let mut best = 0;
    for permutation in permute::permutations_of(&base_config) {
        let cur = run_amplifier_loop(&program, permutation);
        best = std::cmp::max(best, cur);
    }
    best
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples_day07_part1() {
        {
            let program = super::parse_program("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
            assert_eq!(super::find_max_amplifier_config(&program), 43210);
        }
        {
            let program = super::parse_program(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
            );
            assert_eq!(super::find_max_amplifier_config(&program), 54321);
        }
        {
            let program = super::parse_program("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
            assert_eq!(super::find_max_amplifier_config(&program), 65210);
        }
    }

    #[test]
    fn samples_day07_part2() {
        {
            let program = super::parse_program("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
            assert_eq!(super::find_max_amplifier_loop(&program), 139629729);
        }
    }
}

fn main() {
    let program = parse_program("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    assert_eq!(find_max_amplifier_config(&program), 43210);

    let puzzle_input = include_str!("input.txt");
    let program = parse_program(puzzle_input);
    println!("part 1 {:?}", find_max_amplifier_config(&program));
    println!("part 2 {:?}", find_max_amplifier_loop(&program));
}
