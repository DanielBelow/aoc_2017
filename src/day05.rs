use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day05)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.lines().filter_map(|it| it.parse().ok()).collect_vec()
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn count_num_steps<F>(inp: &mut [i64], modify: F) -> i64
where
    F: Fn(i64) -> i64,
{
    let input_len = inp.len() as i64;

    let mut result = 0;
    let mut cur_idx = 0i64;
    while cur_idx >= 0 && cur_idx < input_len {
        assert!(!cur_idx.is_negative());
        let idx = cur_idx as usize;

        let offset = inp[idx];
        inp[idx] = modify(inp[idx]);
        cur_idx += offset;

        result += 1;
    }

    result
}

#[aoc(day05, part1)]
pub fn part1(inp: &[i64]) -> i64 {
    let mut inp = inp.to_owned();
    count_num_steps(&mut inp, |it| it + 1)
}

#[aoc(day05, part2)]
pub fn part2(inp: &[i64]) -> i64 {
    let mut inp = inp.to_owned();
    count_num_steps(&mut inp, |it| if it >= 3 { it - 1 } else { it + 1 })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0\n\
                             3\n\
                             0\n\
                             1\n\
                             -3";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_INPUT);
        let res = part1(&data);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_INPUT);
        let res = part2(&data);
        assert_eq!(res, 10);
    }
}
