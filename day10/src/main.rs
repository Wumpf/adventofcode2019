use std::f64;

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

fn part2_2(field: &mut Vec<Vec<u8>>, station: Point) -> u32 {
    let mut last_laser_angle = f64::consts::FRAC_PI_2 + 0.001;

    let mut count = 0;

    loop {
        let mut any = false;
        let mut cand_angle = -99999.0;
        let mut cand_point = Point{x:99999, y:99999};
        let mut cand_distsq = 999999;
        for y in 0..field.len() {
            for x in 0..field[y].len() {
                if field[y as usize][x as usize] == '#' as u8 {
                    let dx = x as i32 - station.x;
                    let dy = y as i32 - station.y;
                    let distsq = dx * dx + dy * dy;
                    let mut curangle = (-dy as f64).atan2(dx as f64);
                    if curangle < 0.0 {
                        curangle = f64::consts::PI * 2.0 + curangle;
                    }
                    if (curangle < last_laser_angle && curangle > cand_angle) || (curangle == cand_angle && cand_distsq > distsq) {
                        any = true;
                        cand_angle = curangle;
                        cand_distsq = distsq;
                        cand_point = Point{x:x as i32, y:y as i32};
                    }
                }
            }
        }
        if any == false {
            last_laser_angle = f64::consts::PI * 2.0 + 0.000001;
        } else {
            last_laser_angle = cand_angle;
            field[cand_point.y as usize][cand_point.x as usize] = '.' as u8;
            count += 1;
            
            if count == 200 {
                println!("part 2 {}", cand_point.x * 100 + cand_point.y);
            }
        }
    }
}

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
    let mut field = parse_field(puzzle_input);
    let p1 = get_highest_visibility(&field);
    println!("part 1 {}", p1.0);

    part2_2(&mut field, p1.1);
}
