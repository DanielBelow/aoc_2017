use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Instruction {
    #[display("s{0}")]
    Spin(usize),

    #[display("x{0}/{1}")]
    Exchange(usize, usize),

    #[display("p{0}/{1}")]
    Partner(char, char),
}

#[aoc_generator(day16)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.split(',').filter_map(|it| it.parse().ok()).collect()
}

fn get_programs(num_programs: u8) -> Vec<char> {
    (b'a'..(b'a' + num_programs)).map(char::from).collect()
}

fn dance_n_times(
    insts: &[Instruction],
    mut num_dances: usize,
    mut programs: Vec<char>,
) -> Vec<char> {
    let mut moves = (0..programs.len()).collect_vec();
    let mut subs: HashMap<char, char> = programs.iter().map(|it| (*it, *it)).collect();

    for inst in insts {
        match *inst {
            Instruction::Spin(n) => moves.rotate_right(n),
            Instruction::Exchange(l, r) => moves.swap(l, r),
            Instruction::Partner(l, r) => {
                for val in subs.values_mut() {
                    if *val == l {
                        *val = r;
                    } else if *val == r {
                        *val = l;
                    }
                }
            }
        };
    }

    while num_dances > 0 {
        if num_dances % 2 == 1 {
            programs = moves.iter().map(|it| subs[&programs[*it]]).collect_vec();
        }

        moves = moves.iter().map(|it| moves[*it]).collect();
        subs = subs.iter().map(|(key, it)| (*key, subs[it])).collect();

        num_dances /= 2;
    }

    programs
}

fn part1_impl(insts: &[Instruction], num_programs: u8) -> String {
    let letters = get_programs(num_programs);

    dance_n_times(insts, 1, letters)
        .iter()
        .map(ToString::to_string)
        .join("")
}

fn part2_impl(insts: &[Instruction], num_programs: u8) -> String {
    let letters = (b'a'..(b'a' + num_programs)).map(char::from).collect_vec();

    dance_n_times(insts, 1_000_000_000, letters)
        .iter()
        .map(ToString::to_string)
        .join("")
}

#[aoc(day16, part1)]
pub fn part1(insts: &[Instruction]) -> String {
    part1_impl(insts, 16)
}

#[aoc(day16, part2)]
pub fn part2(insts: &[Instruction]) -> String {
    part2_impl(insts, 16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let insts = vec![
            Instruction::Spin(1),
            Instruction::Exchange(3, 4),
            Instruction::Partner('e', 'b'),
        ];

        let programs = get_programs(5);
        let res = dance_n_times(&insts, 1, programs);
        assert_eq!(res, vec!['b', 'a', 'e', 'd', 'c']);
    }

    #[test]
    fn sample_part2() {
        let insts = vec![
            Instruction::Spin(1),
            Instruction::Exchange(3, 4),
            Instruction::Partner('e', 'b'),
        ];

        let programs = get_programs(5);
        let res = dance_n_times(&insts, 2, programs);
        assert_eq!(res, vec!['c', 'e', 'a', 'd', 'b']);
    }
}
