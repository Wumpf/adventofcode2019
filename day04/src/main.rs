fn is_valid_pass_part1(num: i32) -> bool {
    let num_str = num.to_string();
    let num_bytes = num_str.as_bytes();
    if num_bytes.len() != 6 {
        return false;
    }

    let mut any_adjacent_same = false;
    for c in 1..num_bytes.len() {
        if num_bytes[c] < num_bytes[c-1] {
            return false;
        }
        any_adjacent_same |= num_bytes[c] == num_bytes[c-1];
    }

    any_adjacent_same
}

fn is_valid_pass_part2(num: i32) -> bool {
    let num_str = num.to_string();
    let num_bytes = num_str.as_bytes();
    if num_bytes.len() != 6 {
        return false;
    }

    let mut adjacent_rule = false;
    let mut adjacent_count = 0;
    for c in 1..num_bytes.len() {
        if num_bytes[c] < num_bytes[c-1] {
            return false;
        }

        if num_bytes[c] == num_bytes[c-1] {
            adjacent_count += 1;
        } else {
            if adjacent_count == 1 {
                adjacent_rule = true;
            }
            adjacent_count = 0;
        }
    }

    adjacent_rule || adjacent_count == 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples_day03_part1() {
        assert_eq!(super::is_valid_pass_part1(111111), true);
        assert_eq!(super::is_valid_pass_part1(223450), false);
        assert_eq!(super::is_valid_pass_part1(123789), false);
    }

    #[test]
    fn samples_day04_part2() {
        assert_eq!(super::is_valid_pass_part2(112233), true);
        assert_eq!(super::is_valid_pass_part2(123444), false);
        assert_eq!(super::is_valid_pass_part2(111122), true);
    }
}

fn main() {
    let range_min = 136760;
    let range_max = 595730;

    is_valid_pass_part2(112233);

    let mut count_part1 = 0;
    let mut count_part2 = 0;
    for num in range_min..(range_max+1) {
        if is_valid_pass_part1(num) {
            count_part1 += 1;
            if is_valid_pass_part2(num) {
                count_part2 += 1;
            }
        }
    }

    println!("part1: {}", count_part1);
    println!("part2: {}", count_part2);
}

