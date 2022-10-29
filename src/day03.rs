use aoc_runner_derive::{aoc, aoc_generator};
use num_complex::Complex;
use std::collections::HashMap;

#[aoc_generator(day03)]
pub fn generate(inp: &str) -> Option<usize> {
    inp.parse().ok()
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn get_spiral_directions(inp: usize) -> Vec<Direction> {
    let mut result = Vec::new();

    if inp == 1 {
        return result;
    }

    let mut num_steps = 1;
    loop {
        if result.len() >= inp {
            break;
        }

        if num_steps % 2 == 1 {
            result.extend(std::iter::repeat(Direction::Right).take(num_steps));
            result.extend(std::iter::repeat(Direction::Up).take(num_steps));
        } else {
            result.extend(std::iter::repeat(Direction::Left).take(num_steps));
            result.extend(std::iter::repeat(Direction::Down).take(num_steps));
        }

        num_steps += 1;
    }

    result
}

#[aoc(day03, part1)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn part1(inp: &usize) -> i64 {
    let directions = get_spiral_directions(*inp);

    let (mut cur_x, mut cur_y) = (0i64, 0i64);
    let mut cur_num = 1;
    for dir in directions {
        match dir {
            Direction::Up => cur_y += 1,
            Direction::Right => cur_x += 1,
            Direction::Down => cur_y -= 1,
            Direction::Left => cur_x -= 1,
        };

        cur_num += 1;

        if cur_num == *inp {
            break;
        }
    }

    cur_x.abs() + cur_y.abs()
}

fn sum_of_neighbours(pos: Complex<i64>, cache: &HashMap<Complex<i64>, i64>) -> i64 {
    const OFFSETS: [Complex<i64>; 8] = [
        Complex::new(-1, 1),
        Complex::new(0, 1),
        Complex::new(1, 1),
        Complex::new(-1, 0),
        Complex::new(1, 0),
        Complex::new(-1, -1),
        Complex::new(0, -1),
        Complex::new(1, -1),
    ];

    let mut result = 0;
    for offset in OFFSETS {
        let key = pos + offset;
        if let Some(val) = cache.get(&key) {
            result += *val;
        }
    }

    result
}

#[aoc(day03, part2)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn part2(inp: &usize) -> i64 {
    let directions = get_spiral_directions(*inp);
    let mut cur_pos = Complex::new(0i64, 0i64);

    let mut cache = HashMap::new();
    cache.insert(Complex::new(0, 0), 1i64);

    for dir in directions {
        match dir {
            Direction::Up => cur_pos.im += 1,
            Direction::Right => cur_pos.re += 1,
            Direction::Down => cur_pos.im -= 1,
            Direction::Left => cur_pos.re -= 1,
        };

        let sum = sum_of_neighbours(cur_pos, &cache);
        if sum > *inp as i64 {
            return sum;
        }

        cache.insert(cur_pos, sum);
    }

    unreachable!("Didn't hit number larger than input")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inputs = [(1usize, 0i64), (12, 3), (23, 2), (1024, 31)];

        for (inp, expected) in inputs {
            let res = part1(&inp);
            assert_eq!(res, expected);
        }
    }
}
