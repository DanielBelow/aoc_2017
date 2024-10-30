use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::ops::BitXor;

#[aoc_generator(day10, part1)]
pub fn generate(inp: &str) -> Vec<usize> {
    inp.split(',')
        .filter_map(|it| it.parse().ok())
        .collect_vec()
}

fn run_steps(
    lengths: &[usize],
    nums: &mut [usize],
    current_position: &mut usize,
    skip_size: &mut usize,
) {
    let max_num = nums.len();

    for &length in lengths {
        let mut slice = (0..length).fold(Vec::new(), |mut acc, it| {
            let index = *current_position + it;
            acc.push(nums[index % max_num]);
            acc
        });
        slice.reverse();

        assert_eq!(length, slice.len());

        for (idx, elem) in slice.iter().enumerate() {
            let index = *current_position + idx;
            nums[index % max_num] = *elem;
        }

        *current_position += length + *skip_size;
        *skip_size += 1;
    }
}

fn part1_impl(lengths: &[usize], max_num: usize) -> usize {
    let mut nums = (0..=max_num).collect_vec();
    run_steps(lengths, &mut nums, &mut 0, &mut 0);
    nums[0] * nums[1]
}

#[aoc(day10, part1)]
pub fn part1(lengths: &[usize]) -> usize {
    part1_impl(lengths, 255)
}

fn bitxor_as_hex(nums: &[usize]) -> String {
    let result = nums.iter().fold(0, |acc, it| acc.bitxor(*it));
    format!("{result:02x?}")
}

#[aoc(day10, part2)]
pub fn part2(inp: &str) -> String {
    let mut lengths = inp.chars().map(|it| usize::from(it as u8)).collect_vec();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    let mut nums = (0..=255).collect_vec();

    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        run_steps(&lengths, &mut nums, &mut current_position, &mut skip_size);
    }

    nums.chunks(16).map(bitxor_as_hex).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inputs = [("3,4,1,5", 12)];
        for (inp, expected) in inputs {
            let data = generate(inp);
            let res = part1_impl(&data, 4);
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn test_sample_p2() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", part2(""));
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", part2("AoC 2017"));
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", part2("1,2,3"));
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", part2("1,2,4"));
    }
}
