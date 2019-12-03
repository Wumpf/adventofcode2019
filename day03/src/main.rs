use std::cmp;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct WireSegment {
    min: Point,
    max: Point,
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
            min: Point {
                x: cmp::min(old_pos.x, pos.x),
                y: cmp::min(old_pos.y, pos.y),
            },
            max: Point {
                x: cmp::max(old_pos.x, pos.x),
                y: cmp::max(old_pos.y, pos.y),
            },
        });
    }

    segments
}

fn compute_intersections(wiredesc0: &str, wiredesc1: &str) -> Vec<Point> {
    let segments0 = parse_wiredesc(wiredesc0);
    let segments1 = parse_wiredesc(wiredesc1);

    let mut intersections = Vec::new();

    for segment0 in segments0.iter() {
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
                    intersections.push(Point {
                        x: vseg.min.x,
                        y: hseg.min.y,
                    });
                }
            }
        }
    }

    intersections
}

fn compute_maxdistance(wiredesc0: &str, wiredesc1: &str) -> i32 {
    let intersections = compute_intersections(wiredesc0, wiredesc1);
    intersections
        .iter()
        .map(|p| p.x.abs() + p.y.abs())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples_day03_part1() {
        assert_eq!(super::compute_maxdistance("R8,U5,L5,D3", "U7,R6,D4,L4"), 6);
        assert_eq!(
            super::compute_maxdistance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
        assert_eq!(
            super::compute_maxdistance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }
}

fn main() {
    let puzzle_input = include_str!("input.txt");
    let mut lines = puzzle_input.lines();
    println!("part1: {}", compute_maxdistance(lines.next().unwrap(), lines.next().unwrap()));
}
