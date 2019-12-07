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

fn compute_level_depths_rec<'a>(connections: &'a HashMap<&str, Vec<&str>>, depths: &mut HashMap<&'a str, u32>, node: &'a str, cur_depth: u32) {
    depths.insert(node, cur_depth);

    let edges = match connections.get(node) {
        Some(val) => val,
        None => return,
    };
    for edge in edges {
        compute_level_depths_rec(connections, depths, edge, cur_depth + 1);
    }
}

fn compute_level_depths<'a>(connections: &'a HashMap<&str, Vec<&str>>, root: &'a str) -> HashMap<&'a str, u32> {
    let mut depths: HashMap<&'a str, u32> = HashMap::new();
    compute_level_depths_rec(connections, &mut depths, root, 0);
    depths
}

fn common_ancestor<'a>(connections: &'a HashMap<&str, Vec<&str>>, root: &'a str, a: &'a str, b: &'a str) -> Option<&'a str> {
    if root == a {
        return Some(a);
    }
    else if root == b {
        return Some(b);
    }

    let edges = match connections.get(root) {
        Some(val) => val,
        None => return None,
    };
    let nodes: Vec<&str> = edges.iter().filter_map(|edge| common_ancestor(connections, edge, a, b)).collect();
    if nodes.contains(&a) && nodes.contains(&b) {
        Some(root)
    } else if nodes.is_empty() {
        None
    } else {
        Some(nodes[0])
    }
}

fn graph_distance(connections: &HashMap<&str, Vec<&str>>, depths: &HashMap<&str, u32>, root: &str) -> u32 {
    let from = "YOU";
    let to = "SAN";

    let ancestor = common_ancestor(connections, root, from, to);
    depths[from] + depths[to] - depths[ancestor.unwrap()] * 2 - 2
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
        let depths = super::compute_level_depths(&connections, root);
        assert_eq!(depths.values().sum::<u32>(), 42);
    }

    #[test]
    fn samples_day06_part2() {
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
K)L
K)YOU
I)SAN",
        );
        let root = super::find_root(&connections);
        let depths = super::compute_level_depths(&connections, root);
        assert_eq!(super::graph_distance(&connections, &depths, root), 4);
    }
}

fn main() {
    let puzzle_input = include_str!("input.txt");
    let connections = parse_connections(puzzle_input);
    let root = find_root(&connections);
    let depths = compute_level_depths(&connections, root);
    println!("part 1 {}", depths.values().sum::<u32>());
    println!("part 2 {}", graph_distance(&connections, &depths, root));
}
