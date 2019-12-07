fn parse_program(puzzle_input: &str) -> Vec<i32> {
    puzzle_input
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn eval_program(program: &mut Vec<i32>, input: i32) -> Vec<i32> {
    let mut pc = 0; // program counter
    let mut output = Vec::new();
    loop {
        let opcode = program[pc] % 100;
        let pmodes = [
            program[pc] / 100 % 10,
            program[pc] / 1000 % 10,
            program[pc] / 10000 % 10,
        ];
        assert_eq!(pmodes[2], 0);

        let get_param = |p| {
            if pmodes[p] == 0 {
                program[program[pc + p + 1] as usize]
            } else {
                program[pc + p + 1]
            }
        };

        match opcode {
            1 => {
                let tidx = program[pc + 3] as usize;
                program[tidx] = get_param(0) + get_param(1)
            }
            2 => {
                let tidx = program[pc + 3] as usize;
                program[tidx] = get_param(0) * get_param(1)
            }
            3 => {
                let tidx = program[pc + 1] as usize;
                program[tidx] = input
            }
            4 => output.push(get_param(0)),
            5 => {
                if get_param(0) != 0 {
                    pc = get_param(1) as usize;
                    continue;
                }
            }
            6 => {
                if get_param(0) == 0 {
                    pc = get_param(1) as usize;
                    continue;
                }
            }
            7 => {
                let tidx = program[pc + 3] as usize;
                program[tidx] = if get_param(0) < get_param(1) { 1 } else { 0 }
            }
            8 => {
                let tidx = program[pc + 3] as usize;
                program[tidx] = if get_param(0) == get_param(1) { 1 } else { 0 }
            }
            99 => break,
            _ => panic!("Invalid opcode!"),
        };

        pc += match opcode {
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

    output
}

fn parse_and_eval_program(puzzle_input: &str, input: i32) -> Vec<i32> {
    let mut program = parse_program(puzzle_input);
    eval_program(&mut program, input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples_day05_part2() {
        assert_eq!(
            super::parse_and_eval_program("3,9,8,9,10,9,4,9,99,-1,8", 8),
            [1]
        );
        assert_eq!(
            super::parse_and_eval_program("3,9,8,9,10,9,4,9,99,-1,8", 7),
            [0]
        );
        assert_eq!(
            super::parse_and_eval_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 8),
            [1]
        );
        assert_eq!(
            super::parse_and_eval_program("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0),
            [0]
        );
        assert_eq!(
            super::parse_and_eval_program("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", -1),
            [1]
        );
        assert_eq!(
            super::parse_and_eval_program("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0),
            [0]
        );
        assert_eq!(super::parse_and_eval_program("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 0), [999]);
        assert_eq!(super::parse_and_eval_program("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 8), [1000]);
        assert_eq!(super::parse_and_eval_program("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 123), [1001]);
    }
}

fn main() {
    let puzzle_input = include_str!("input.txt");
    //parse_and_eval_program("1002,4,3,4,33", 0);

    println!("part 1 {:?}", parse_and_eval_program(puzzle_input, 1));
    println!("part 2 {:?}", parse_and_eval_program(puzzle_input, 5));
}
