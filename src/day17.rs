use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
pub fn generate(inp: &str) -> Option<usize> {
    inp.parse().ok()
}

#[aoc(day17, part1)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn part1(num: &usize) -> usize {
    let mut buffer = vec![0usize];

    let mut cur_pos = 0;
    for to_insert in 1..=2017 {
        cur_pos = ((cur_pos + *num) % buffer.len()) + 1;

        if cur_pos == buffer.len() {
            buffer.push(to_insert);
        } else {
            buffer.insert(cur_pos, to_insert);
        }
    }

    buffer[(cur_pos + 1) % buffer.len()]
}

#[aoc(day17, part2)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn part2(num: &usize) -> usize {
    let mut at_index_1 = 1;

    let mut buffer_len = 1;
    let mut cur_pos = 0;

    for to_insert in 1..=50_000_000 {
        cur_pos = ((cur_pos + *num) % buffer_len) + 1;

        if cur_pos == 1 {
            at_index_1 = to_insert;
        }

        buffer_len += 1;
    }

    at_index_1
}
