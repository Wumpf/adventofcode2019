fn parse_program(puzzle_input: &str) -> Vec<i32> {
    puzzle_input
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

// How to do this idiomatic?
//fn foo(a: &mut Vec<usize>) {
//    a[a[0]] = 1;
//}

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
            99 => break,
            _ => panic!("Invalid opcode!"),
        };

        pc += match opcode {
            1 => 4,
            2 => 4,
            3 => 2,
            4 => 2,
            _ => panic!("Invalid opcode!"),
        };
    }

    output
}

#[allow(dead_code)]
fn parse_and_eval_program(puzzle_input: &str, input: i32) -> Vec<i32> {
    let mut program = parse_program(puzzle_input);
    eval_program(&mut program, input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples_day02_part1() {}
}

fn main() {
    let puzzle_input = include_str!("input.txt");
    //parse_and_eval_program("1002,4,3,4,33", 0);

    println!("{:?}", parse_and_eval_program(puzzle_input, 1));
}
