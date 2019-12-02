fn parse_program(puzzle_input: &str) -> Vec<usize> {
    puzzle_input
        .split(',')
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn eval_program(program: &mut Vec<usize>) {
    let mut ppos = 0;
    while program[ppos] != 99 {
        let op0 = program[program[ppos + 1]];
        let op1 = program[program[ppos + 2]];
        let target = program[ppos + 3];
        program[target] = match program[ppos] {
            1 => op0 + op1,
            2 => op0 * op1,
            _ => panic!("Invalid opcode!")
        };
        ppos += 4;
    }
}

#[warn(dead_code)]
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

    let mut program = parse_program(puzzle_input);
    program[1] = 12;
    program[2] = 2;
    eval_program(&mut program);
    println!("part1 {}", program[0])
}
