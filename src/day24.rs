use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Eq, Copy, Clone, Debug)]
#[display("{lhs}/{rhs}")]
pub struct Node {
    lhs: i32,
    rhs: i32,
}

#[aoc_generator(day24)]
pub fn generate(inp: &str) -> Vec<Node> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

fn path_strength(path: &[Node]) -> i32 {
    path.iter().map(|it| it.lhs + it.rhs).sum()
}

fn build_paths(
    cur_node: Node,
    left_is_free: bool,
    available_nodes: &[Node],
    cur_path: &mut [Node],
    paths: &mut Vec<Vec<Node>>,
) {
    let free_connector = if left_is_free {
        cur_node.lhs
    } else {
        cur_node.rhs
    };

    let candidates = available_nodes
        .iter()
        .filter(|it| it.lhs == free_connector || it.rhs == free_connector)
        .collect_vec();

    if candidates.is_empty() {
        paths.push(cur_path.to_vec());
        return;
    }

    for candidate in candidates {
        let rem_nodes = available_nodes
            .iter()
            .filter(|it| **it != *candidate)
            .copied()
            .collect_vec();

        let mut cur_path = cur_path.to_vec();
        cur_path.push(*candidate);

        build_paths(
            *candidate,
            candidate.rhs == free_connector,
            &rem_nodes,
            &mut cur_path,
            paths,
        );
    }
}

fn find_paths(inp: &[Node]) -> Vec<Vec<Node>> {
    let nodes = inp.to_vec();

    let start_nodes = nodes
        .iter()
        .filter(|it| it.lhs == 0 || it.rhs == 0)
        .collect_vec();

    let mut paths = vec![];

    for start_node in start_nodes {
        let rem_nodes = nodes
            .iter()
            .filter(|it| **it != *start_node)
            .copied()
            .collect_vec();
        build_paths(
            *start_node,
            start_node.rhs == 0,
            &rem_nodes,
            &mut [*start_node],
            &mut paths,
        );
    }

    paths
}

#[aoc(day24, part1)]
pub fn part1(inp: &[Node]) -> Option<i32> {
    find_paths(inp).iter().map(|it| path_strength(it)).max()
}

#[aoc(day24, part2)]
pub fn part2(inp: &[Node]) -> Option<i32> {
    let all_paths = find_paths(inp);
    let longest_paths = all_paths.iter().max_set_by_key(|it| it.len());

    longest_paths.iter().map(|it| path_strength(it)).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "0/2\n\
                             2/2\n\
                             2/3\n\
                             3/4\n\
                             3/5\n\
                             0/1\n\
                             10/1\n\
                             9/10";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_DATA);
        let result = part1(&data);
        assert_eq!(result.unwrap(), 31);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_DATA);
        let result = part2(&data);
        assert_eq!(result.unwrap(), 19);
    }
}
