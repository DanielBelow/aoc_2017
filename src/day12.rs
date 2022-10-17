use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day12)]
pub fn generate(inp: &str) -> Vec<HashSet<usize>> {
    let connections = inp
        .lines()
        .map(|it| {
            let split = it.split(" <-> ").collect_vec();
            assert_eq!(split.len(), 2);
            let lhs = split[0].parse::<usize>().unwrap();

            let rhs = split[1]
                .split(", ")
                .map(|it| it.parse::<usize>().unwrap())
                .collect_vec();

            assert!(!rhs.is_empty());

            (lhs, rhs)
        })
        .collect::<HashMap<_, _>>();

    pathfinding::undirected::connected_components::connected_components(
        &connections.keys().copied().collect_vec(),
        |it| connections[it].clone(),
    )
}

#[aoc(day12, part1)]
pub fn part1(components: &[HashSet<usize>]) -> Option<usize> {
    components
        .iter()
        .find(|it| it.contains(&0))
        .map(HashSet::len)
}

#[aoc(day12, part2)]
pub fn part2(components: &[HashSet<usize>]) -> usize {
    components.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inp = "0 <-> 2\n\
                          1 <-> 1\n\
                          2 <-> 0, 3, 4\n\
                          3 <-> 2, 4\n\
                          4 <-> 2, 3, 6\n\
                          5 <-> 6\n\
                          6 <-> 4, 5";
        let data = generate(inp);
        let result = part1(&data);

        assert_eq!(result.unwrap(), 6);
    }

    #[test]
    fn test_sample_p2() {
        let inp = "0 <-> 2\n\
                          1 <-> 1\n\
                          2 <-> 0, 3, 4\n\
                          3 <-> 2, 4\n\
                          4 <-> 2, 3, 6\n\
                          5 <-> 6\n\
                          6 <-> 4, 5";
        let data = generate(inp);
        let result = part2(&data);

        assert_eq!(result, 2);
    }
}
