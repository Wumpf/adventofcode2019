use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
type Dir = Point;

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

fn print_panels(panels: &HashMap<Point, i64>) {
    let mut minx = 9999;
    let mut miny = 9999;
    let mut maxx = -9999;
    let mut maxy = -9999;
    for (p,c) in panels {
        if *c == 1 {
            minx = std::cmp::min(minx, p.x);
            miny = std::cmp::min(miny, p.y);
            maxx = std::cmp::max(maxx, p.x);
            maxy = std::cmp::max(maxy, p.y);
        }
    }

    for y in miny..(maxy+1) {
        for x in minx..(maxx+1) {
            let c = match panels.get(&Point{x:x, y:y}) {
                Some(0) => ".",
                Some(1) => "█",
                None => " ",
                _ => panic!(),
            };
            print!("{}", c);
        }
        println!();
    }
}

fn run_robot(puzzle_input: &str, start_white: bool) -> usize {
    let mut program = ProgramState {
        program: parse_program(puzzle_input),
        inputs: VecDeque::new(),
        pc: 0,
        relative_base: 0,
    };

    let mut pos = Point{x:0, y:0};
    let mut dir = 0;
    let dirs = [Dir{x:0, y:-1}, Dir{x:1, y:0}, Dir{x:0, y:1}, Dir{x:-1, y:0}];
    let mut panels = HashMap::new();
    if start_white {
        panels.insert(pos, 1);
    }
    loop {
        program.inputs.push_back(*panels.get(&pos).unwrap_or(&0));

        let color = match program.eval_program() {
            ProgramResult::Output(out) => out,
            ProgramResult::Halted =>{
                print_panels(&panels);
                return panels.len();
            },
            _ => panic!("invalid program state"),
        };
        dir += match program.eval_program() {
            ProgramResult::Output(out) => out * 2 - 1,
            _ => panic!("invalid program state"),
        };
        if dir < 0 {
            dir += 4;
        } else if dir > 3 {
            dir -= 4;
        }

        panels.insert(pos, color);

        pos = Point{x:pos.x+dirs[dir as usize].x, y:pos.y+dirs[dir as usize].y};
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples_day11_part1() {
    }

    #[test]
    fn samples_day11_part2() {}
}

fn main() {
    let puzzle_input = include_str!("input.txt");
    println!("part 1 {}", run_robot(puzzle_input, false));

    println!("part 2");
    run_robot(puzzle_input, true);
}
