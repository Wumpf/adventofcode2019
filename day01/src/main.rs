fn compute_fuel_from_mass_part1(mass: i32) -> i32 {
    mass / 3 - 2
}

fn compute_fuel_from_mass_part2(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        fuel + compute_fuel_from_mass_part2(fuel)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples_day01_part1() {
        assert_eq!(super::compute_fuel_from_mass_part1(12), 2);
        assert_eq!(super::compute_fuel_from_mass_part1(14), 2);
        assert_eq!(super::compute_fuel_from_mass_part1(1969), 654);
        assert_eq!(super::compute_fuel_from_mass_part1(100756), 33583);
    }

    #[test]
    fn samples_day01_part2() {
        assert_eq!(super::compute_fuel_from_mass_part2(14), 2);
        assert_eq!(super::compute_fuel_from_mass_part2(1969), 966);
        assert_eq!(super::compute_fuel_from_mass_part2(100756), 50346);
    }
}

fn main() {
    let puzzle_input = include_str!("input.txt");
    let modules_mass = puzzle_input
        .lines()
        .map(|line| line.parse::<i32>().unwrap());

    let mut total_fuel_part1 = 0;
    let mut total_fuel_part2 = 0;
    for mass in modules_mass {
        total_fuel_part1 += compute_fuel_from_mass_part1(mass);
        total_fuel_part2 += compute_fuel_from_mass_part2(mass);
    }
    println!("part1 {}", total_fuel_part1);
    println!("part2 {}", total_fuel_part2);
}
