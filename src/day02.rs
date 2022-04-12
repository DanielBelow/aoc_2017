use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn parse_row(line: &str) -> Vec<u32> {
    line.split_ascii_whitespace()
        .filter_map(|it| it.trim().parse().ok())
        .collect()
}

#[aoc_generator(day02)]
pub fn generate(inp: &str) -> Vec<Vec<u32>> {
    inp.lines().map(parse_row).collect()
}

#[aoc(day02, part1)]
pub fn part1(inp: &[Vec<u32>]) -> u32 {
    inp.iter()
        .filter_map(|it| {
            if let Some((min, max)) = it.iter().minmax().into_option() {
                Some(*max - *min)
            } else {
                None
            }
        })
        .sum()
}

fn divides(lhs: u32, rhs: u32) -> bool {
    let divides = |lhs, rhs| -> bool { lhs > rhs && lhs % rhs == 0 };
    divides(lhs, rhs) || divides(rhs, lhs)
}

#[aoc(day02, part2)]
pub fn part2(inp: &[Vec<u32>]) -> u32 {
    let mut result = 0;

    for row in inp {
        let (&lhs, &rhs) = row
            .iter()
            .cartesian_product(row)
            .filter(|&(lhs, rhs)| lhs != rhs)
            .find(|&(lhs, rhs)| divides(*lhs, *rhs))
            .expect("Expecting two elements that divide each other");

        result += lhs.max(rhs) / lhs.min(rhs);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let input: &str = "5 1 9 5\n\
                           7 5 3\n\
                           2 4 6 8";
        let data = generate(input);
        let res = part1(&data);
        assert_eq!(res, 18);
    }

    #[test]
    fn test_sample_p2() {
        let input: &str = "5 9 2 8\n\
                           9 4 7 3\n\
                           3 8 6 5";
        let data = generate(input);
        let res = part2(&data);
        assert_eq!(res, 9);
    }
}
