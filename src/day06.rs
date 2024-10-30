use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day06)]
pub fn generate(inp: &str) -> Vec<usize> {
    inp.split_ascii_whitespace()
        .filter_map(|it| it.trim().parse().ok())
        .collect_vec()
}

fn find_first_max_position(slice: &[usize]) -> Option<(usize, &usize)> {
    let max_elem = slice.iter().max()?;
    slice.iter().find_position(|it| **it == *max_elem)
}

fn redistribute_blocks(slots: &mut [usize]) -> Option<()> {
    let num_slots = slots.len();

    // find most blocks
    let (most_blocks, _) = find_first_max_position(slots)?;
    let mut num_blocks = slots[most_blocks];

    // take all blocks
    slots[most_blocks] = 0;

    // redistribute
    let mut next_idx = most_blocks + 1;
    while num_blocks > 0 {
        slots[next_idx % num_slots] += 1;
        num_blocks -= 1;
        next_idx += 1;
    }

    Some(())
}

#[aoc(day06, part1)]
pub fn part1(inp: &[usize]) -> Option<usize> {
    let mut inp = inp.to_owned();

    let mut cache = HashMap::new();
    cache.insert(inp.clone(), 0);

    for step in 1.. {
        redistribute_blocks(&mut inp)?;
        if cache.contains_key(&inp) {
            return Some(step);
        }

        cache.insert(inp.clone(), step);
    }

    unreachable!()
}

#[aoc(day06, part2)]
pub fn part2(inp: &[usize]) -> Option<usize> {
    let mut inp = inp.to_owned();

    let mut cache = HashMap::new();
    cache.insert(inp.clone(), 0);

    for step in 1.. {
        redistribute_blocks(&mut inp)?;
        if let Some(prev) = cache.get(&inp) {
            return Some(step - *prev);
        }

        cache.insert(inp.clone(), step);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0 2 7 0";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, Some(5));
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res, Some(4));
    }
}
