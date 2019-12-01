fn compute_fuel_from_mass(mass: i32) -> i32 {
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples_part1() {
        assert_eq!(super::compute_fuel_from_mass(12), 2);
        assert_eq!(super::compute_fuel_from_mass(14), 2);
        assert_eq!(super::compute_fuel_from_mass(1969), 654);
        assert_eq!(super::compute_fuel_from_mass(100756), 33583);
    }
}

fn main() {
    let puzzle_input = include_str!("input.txt");

    let total_fuel: i32 = puzzle_input
        .lines()
        .map(|line| compute_fuel_from_mass(line.parse::<i32>().unwrap()))
        .sum();
    println!("total fuel use is {}", total_fuel);
}
