use std::collections::HashMap;
use std::collections::HashSet;

fn parse_connections(puzzle_input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();

    for line in puzzle_input.lines() {
        let mut parts = line.split(')');
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        let target_list = graph.entry(from).or_insert(Vec::new());
        target_list.push(to);
    }

    graph
}

fn find_root<'a>(connections: &'a HashMap<&str, Vec<&str>>) -> &'a str {
    let mut candidates: HashSet<&'a str> = connections.keys().map(|k| *k).collect();
    for edges in connections.values() {
        for target in edges {
            candidates.remove(target);
        }
    }

    candidates.iter().next().unwrap()
}

fn sum_of_level_depths(connections: &HashMap<&str, Vec<&str>>, node: &str, depth: u32) -> u32 {
    let edges = match connections.get(node) {
        Some(val) => val,
        None => return depth,
    };

    let mut count = depth;
    for edge in edges {
        count += sum_of_level_depths(connections, edge, depth + 1);
    }

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn samples_day06_part1() {
        let connections = super::parse_connections(
            "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L",
        );
        let root = super::find_root(&connections);
        assert_eq!(root, "COM");
        assert_eq!(super::sum_of_level_depths(&connections, root, 0), 42);
    }
}

fn main() {
    let puzzle_input = include_str!("input.txt");
    let connections = parse_connections(puzzle_input);
    let root = find_root(&connections);
    println!("part 1 {}", sum_of_level_depths(&connections, root, 0));
}
