use std::cmp;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct WireSegment {
    start: Point,
    end: Point,
    min: Point,
    max: Point,
    len: i32
}

struct Intersection {
    point: Point,
    signaldelay: i32,
}

fn parse_wiredesc(wiredesc: &str) -> Vec<WireSegment> {
    let mut pos = Point { x: 0, y: 0 };
    let mut segments = Vec::new();
    for step in wiredesc.split(',') {
        let step_len = step[1..].parse::<i32>().unwrap();
        let old_pos = pos;

        match step.chars().next().unwrap() {
            'R' => pos.x += step_len,
            'L' => pos.x -= step_len,
            'U' => pos.y += step_len,
            'D' => pos.y -= step_len,
            _ => panic!(),
        }

        segments.push(WireSegment {
            start: old_pos,
            end: pos,
            min: Point {
                x: cmp::min(old_pos.x, pos.x),
                y: cmp::min(old_pos.y, pos.y),
            },
            max: Point {
                x: cmp::max(old_pos.x, pos.x),
                y: cmp::max(old_pos.y, pos.y),
            },
            len: step_len,
        });
    }

    segments
}

fn compute_intersections(wiredesc0: &str, wiredesc1: &str) -> Vec<Intersection> {
    let segments0 = parse_wiredesc(wiredesc0);
    let segments1 = parse_wiredesc(wiredesc1);

    let mut intersections = Vec::new();

    let mut dist0 = 0;
    for segment0 in segments0.iter() {
        let mut dist1 = 0;
        for segment1 in segments1.iter() {
            let is_segment0_horizontal = segment0.min.y == segment0.max.y;
            let is_segment1_horizontal = segment1.min.y == segment1.max.y;

            // Horizontal overlap
            if is_segment0_horizontal && is_segment1_horizontal {
                // .. ignored
            // Vertical overlap
            } else if !is_segment0_horizontal && !is_segment1_horizontal {
                // .. ignored
            // Horizontal/vertical intersection
            } else {
                let hseg = if is_segment0_horizontal { segment0 } else { segment1 };
                let vseg = if is_segment0_horizontal { segment1 } else { segment0 };
                if !(vseg.min.x == 0 && hseg.min.y == 0)
                    && hseg.min.x <= vseg.min.x
                    && hseg.max.x >= vseg.min.x
                    && hseg.min.y >= vseg.min.y
                    && hseg.min.y <= vseg.max.y
                {
                    intersections.push(Intersection {
                        point: Point {
                            x: vseg.min.x,
                            y: hseg.min.y,
                        },
                        signaldelay: dist0 + dist1 + (hseg.min.y - vseg.start.y).abs() + (vseg.min.x - hseg.start.x).abs()
                    });
                }
            }

            dist1 += segment1.len;
        }
        dist0 += segment0.len;
    }

    intersections
}

fn compute_maxdistance(wiredesc0: &str, wiredesc1: &str) -> i32 {
    let intersections = compute_intersections(wiredesc0, wiredesc1);
    intersections
        .iter()
        .map(|p| p.point.x.abs() + p.point.y.abs())
        .min()
        .unwrap()
}

fn compute_minsignaldelay(wiredesc0: &str, wiredesc1: &str) -> i32 {
    let intersections = compute_intersections(wiredesc0, wiredesc1);
    intersections
        .iter()
        .map(|p| p.signaldelay)
        .min()
        .unwrap()
}


#[cfg(test)]
mod tests {
    #[test]
    fn samples_day03_part1() {
        assert_eq!(super::compute_maxdistance("R8,U5,L5,D3", "U7,R6,D4,L4"), 6);
        assert_eq!(super::compute_maxdistance("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"), 159);
        assert_eq!(super::compute_maxdistance("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 135);
    }

    #[test]
    fn samples_day03_part2() {
        assert_eq!(super::compute_minsignaldelay("R8,U5,L5,D3", "U7,R6,D4,L4"), 30);
        assert_eq!(super::compute_minsignaldelay("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"), 610);
        assert_eq!(super::compute_minsignaldelay("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 410);
    }
}

fn main() {
    let puzzle_input = include_str!("input.txt");
    let mut lines = puzzle_input.lines();

    let wire0 = lines.next().unwrap();
    let wire1 = lines.next().unwrap();

    println!("part1: {}", compute_maxdistance(wire0, wire1));
    println!("part2: {}", compute_minsignaldelay(wire0, wire1));
}
