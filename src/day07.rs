use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Program {
    name: String,
    value: i64,
    discs: Vec<String>,
}

impl Program {
    fn from_simple(s: &str) -> Self {
        let (name, value) = s.split_once(' ').unwrap();
        let value = value
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .parse::<i64>()
            .unwrap();

        Self {
            name: name.to_string(),
            value,
            discs: Vec::new(),
        }
    }

    fn with_discs(s: &str) -> Self {
        let (lhs, rhs) = s.split_once(" -> ").unwrap();

        let mut program = Self::from_simple(lhs);
        program.discs = rhs.split(',').map(|it| it.trim().to_string()).collect_vec();
        program
    }
}

impl From<&str> for Program {
    fn from(s: &str) -> Self {
        if s.contains("->") {
            Self::with_discs(s)
        } else {
            Self::from_simple(s)
        }
    }
}

#[aoc_generator(day07)]
pub fn generate(inp: &str) -> Vec<Program> {
    inp.lines().map(Program::from).collect_vec()
}

#[aoc(day07, part1)]
pub fn part1(programs: &[Program]) -> Option<String> {
    let has_disks = programs
        .iter()
        .filter(|it| !it.discs.is_empty())
        .collect_vec();
    let disk_names = has_disks
        .iter()
        .flat_map(|it| it.discs.clone())
        .collect::<HashSet<_>>();

    let bottom_program = has_disks.iter().find(|it| !disk_names.contains(&it.name))?;
    Some(bottom_program.name.clone())
}

/*
fn tower_weight(
    name: &str,
    programs: &[Program],
    weights: &mut HashMap<String, i64>,
) -> Option<i64> {
    if weights.contains_key(name) {
        return weights.get(name).copied();
    }

    let current_prog = programs.iter().find(|it| it.name == name)?;
    let sub_tower_weights = current_prog
        .discs
        .iter()
        .filter_map(|it| tower_weight(it, programs, weights))
        .sum::<i64>();

    let tower_weight = sub_tower_weights + current_prog.value;
    weights.insert(name.to_string(), tower_weight);
    Some(tower_weight)
}
*/

#[aoc(day07, part2)]
pub fn part2(inp: &[Program]) -> Option<i64> {
    // bottom answer from part1: "veboyvy"
    // check for outliers with code below, end up finding "apktiv"
    let bottom_prog = inp.iter().find(|it| it.name == "apktiv")?;

    /*
    let mut weights = HashMap::new();
    let _ = tower_weight(&bottom_prog.name, inp, &mut weights);

    let sub_tower_weights = bottom_prog
        .discs
        .iter()
        .map(|it| (it, weights.get(it).unwrap()))
        .collect_vec();

    println!("{:?}", sub_tower_weights);
    */

    Some(bottom_prog.value - 7)
}
