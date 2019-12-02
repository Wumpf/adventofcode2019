fn parse_program(puzzle_input: &str) -> Vec<usize> {
    puzzle_input
        .split(',')
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn eval_program(program: &mut Vec<usize>) {
    let mut instrp = 0;
    while program[instrp] != 99 {
        let param0 = program[program[instrp + 1]];
        let param1 = program[program[instrp + 2]];
        let target_address = program[instrp + 3];
        program[target_address] = match program[instrp] {
            1 => param0 + param1,
            2 => param0 * param1,
            _ => panic!("Invalid opcode!")
        };
        instrp += 4;
    }
}

#[allow(dead_code)]
fn parse_and_eval_program(puzzle_input: &str) -> Vec<usize> {
    let mut program = parse_program(puzzle_input);
    eval_program(&mut program);
    program
}


#[cfg(test)]
mod tests {
    #[test]
    fn samples_day02_part1() {
        assert_eq!(super::parse_and_eval_program("1,0,0,0,99"), [2,0,0,0,99]);
        assert_eq!(super::parse_and_eval_program("2,3,0,3,99"), [2,3,0,6,99]);
        assert_eq!(super::parse_and_eval_program("2,4,4,5,99,0"), [2,4,4,5,99,9801]);
        assert_eq!(super::parse_and_eval_program("1,1,1,4,99,5,6,0,99"), [30,1,1,4,2,5,6,0,99]);
    }
}

fn main() {
    let puzzle_input = include_str!("input.txt");
    let original_program = parse_program(puzzle_input);

    {
        let mut program = original_program.clone();
        program[1] = 12;
        program[2] = 2;
        eval_program(&mut program);
        println!("part1 {}", program[0])
    }

    'noun: for noun in 0..100 {
        'verb: for verb in 0..100 {
            let mut program = original_program.clone();
            program[1] = noun;
            program[2] = verb;
            eval_program(&mut program);
            if program[0] == 19690720 {
                println!("part2 {}", 100 * noun + verb);
                break 'noun;
            }
        }
    }
}
