use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(FromStr, Display, Clone, Debug)]
pub enum Condition {
    #[display("{0} == {1}")]
    Eq(String, i64),

    #[display("{0} != {1}")]
    NEq(String, i64),

    #[display("{0} > {1}")]
    Gt(String, i64),

    #[display("{0} < {1}")]
    Lt(String, i64),

    #[display("{0} >= {1}")]
    GEq(String, i64),

    #[display("{0} <= {1}")]
    LEq(String, i64),
}

impl Condition {
    fn evaluate(&self, registers: &mut HashMap<String, i64>) -> bool {
        match self {
            Self::Eq(reg, val) => *registers.entry(reg.clone()).or_insert(0) == *val,
            Self::NEq(reg, val) => *registers.entry(reg.clone()).or_insert(0) != *val,
            Self::Gt(reg, val) => *registers.entry(reg.clone()).or_insert(0) > *val,
            Self::Lt(reg, val) => *registers.entry(reg.clone()).or_insert(0) < *val,
            Self::GEq(reg, val) => *registers.entry(reg.clone()).or_insert(0) >= *val,
            Self::LEq(reg, val) => *registers.entry(reg.clone()).or_insert(0) <= *val,
        }
    }
}

#[derive(FromStr, Display, Clone, Debug)]
pub enum Operation {
    #[display("{0} inc {1}")]
    Inc(String, i64),

    #[display("{0} dec {1}")]
    Dec(String, i64),
}

impl Operation {
    fn evaluate(&self, registers: &mut HashMap<String, i64>) {
        match self {
            Self::Inc(name, val) => *registers.entry(name.clone()).or_insert(0) += *val,
            Self::Dec(name, val) => *registers.entry(name.clone()).or_insert(0) -= *val,
        }
    }
}

#[derive(FromStr, Display, Clone, Debug)]
#[display("{operation} if {condition}")]
pub struct Instruction {
    operation: Operation,
    condition: Condition,
}

impl Instruction {
    fn evaluate(&self, registers: &mut HashMap<String, i64>) {
        if self.condition.evaluate(registers) {
            self.operation.evaluate(registers);
        }
    }
}

#[aoc_generator(day08)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.lines().filter_map(|it| it.parse().ok()).collect_vec()
}

#[aoc(day08, part1)]
pub fn part1(insts: &[Instruction]) -> Option<i64> {
    let mut registers = HashMap::new();

    insts.iter().for_each(|it| it.evaluate(&mut registers));
    registers.values().max().copied()
}

#[aoc(day08, part2)]
pub fn part2(insts: &[Instruction]) -> i64 {
    let mut registers = HashMap::new();

    let mut max_ever = 0;
    for inst in insts {
        inst.evaluate(&mut registers);
        if let Some(cur_max) = registers.values().max() {
            max_ever = max_ever.max(*cur_max);
        }
    }

    max_ever
}
