#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn is_visible(field: &Vec<Vec<u8>>, from: Point, to: Point) -> bool {
    if from == to || field[to.y as usize][to.x as usize] != '#' as u8 {
        return false;
    }

    let dx = to.x - from.x;
    let dy = to.y - from.y;

    for s in (std::cmp::min(0, dx) + 1)..std::cmp::max(0, dx) {
        if s * dy % dx == 0 {
            let x = s + from.x;
            let y = s * dy / dx + from.y;
            if x > 0 && y > 0 && x < field[0].len() as i32 && y < field.len() as i32 {
                if field[y as usize][x as usize] == '#' as u8 {
                    return false;
                }
            }
        }
    }

    if dx == 0 {
        for y in (std::cmp::min(from.y, to.y) + 1)..std::cmp::max(from.y, to.y) {
            if field[y as usize][from.x as usize] == '#' as u8 {
                return false;
            }
        }
    }

    true
}

fn count_visibilties(field: &Vec<Vec<u8>>, p: Point) -> u32 {
    let mut count = 0;
    for y in 0..field.len() {
        for x in 0..field[y].len() {
            if is_visible(
                field,
                p,
                Point {
                    x: x as i32,
                    y: y as i32,
                },
            ) {
                count += 1;
            }
        }
    }
    count
}

fn get_highest_visibility(field: &Vec<Vec<u8>>) -> (u32, Point) {
    let mut max = 0;
    let mut maxp = Point { x: 0, y: 0 };
    for y in 0..field.len() {
        for x in 0..field[y].len() {
            if field[y][x] == '#' as u8 {
                let v = count_visibilties(
                    field,
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                );
                if v > max {
                    max = v;
                    maxp = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                }
            }
        }
    }
    (max, maxp)
}

// fn part2(field: &mut Vec<Vec<u8>>, station: Point) -> u32 {
//     0

// }

fn parse_field(field: &str) -> Vec<Vec<u8>> {
    field.lines().map(|line| line.bytes().collect()).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples_day10_part1() {
        assert_eq!(
            super::get_highest_visibility(&super::parse_field(
                ".#..#
.....
#####
....#
...##"
            ))
            .0,
            8
        );

        assert_eq!(
            super::get_highest_visibility(&super::parse_field(
                ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
            ))
            .0,
            41
        );

        assert_eq!(
            super::get_highest_visibility(&super::parse_field(
                ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
            ))
            .0,
            210
        );
    }

    #[test]
    fn samples_day10_part2() {}
}

fn main() {
    let puzzle_input = include_str!("input.txt");
    let p1 = get_highest_visibility(&parse_field(puzzle_input));
    println!("part 1 {}", p1.0);
}
