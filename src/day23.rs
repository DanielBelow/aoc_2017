use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};

fn reg_to_index(reg: char) -> usize {
    (reg as u8 - b'a') as usize
}

struct EvaluationContext {
    program_counter: usize,
    executed_mul: usize,
}

#[derive(Display, FromStr, Debug)]
pub enum Value {
    #[from_str(regex = "(?P<0>[\\-0-9]+)")]
    #[display("Immediate = {0}")]
    Immediate(i64),

    #[from_str(regex = "(?P<0>[A-Za-z])")]
    #[display("Register = {0}")]
    Register(char),
}

impl Value {
    fn evaluate(&self, registers: &[i64]) -> i64 {
        match self {
            Value::Immediate(val) => *val,
            Value::Register(r) => registers[reg_to_index(*r)],
        }
    }
}

#[derive(Display, FromStr, Debug)]
pub enum Instruction {
    #[display("set {reg} {value}")]
    Set { reg: char, value: Value },

    #[display("sub {reg} {value}")]
    Sub { reg: char, value: Value },

    #[display("mul {reg} {value}")]
    Mul { reg: char, value: Value },

    #[display("jnz {cond} {offset}")]
    Jnz { cond: Value, offset: i64 },
}

impl Instruction {
    fn evaluate(&self, registers: &mut [i64; 8], context: &mut EvaluationContext) {
        match self {
            Instruction::Set { reg, value } => {
                registers[reg_to_index(*reg)] = value.evaluate(registers);
                context.program_counter += 1;
            }
            Instruction::Sub { reg, value } => {
                registers[reg_to_index(*reg)] -= value.evaluate(registers);
                context.program_counter += 1;
            }
            Instruction::Mul { reg, value } => {
                context.executed_mul += 1;
                registers[reg_to_index(*reg)] *= value.evaluate(registers);
                context.program_counter += 1;
            }
            Instruction::Jnz { cond, offset } => {
                context.program_counter += if cond.evaluate(registers) == 0 {
                    1
                } else {
                    usize::try_from(*offset).unwrap()
                }
            }
        }
    }
}

#[aoc_generator(day23)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.lines().filter_map(|it| it.parse().ok()).collect_vec()
}

#[aoc(day23, part1)]
pub fn part1(insts: &[Instruction]) -> usize {
    let mut registers = [0i64; 8];
    let mut context = EvaluationContext {
        program_counter: 0,
        executed_mul: 0,
    };

    while context.program_counter < insts.len() {
        let cur_inst = &insts[context.program_counter];
        cur_inst.evaluate(&mut registers, &mut context);
    }

    context.executed_mul
}

#[allow(clippy::cast_lossless, clippy::cast_possible_truncation)]
fn is_prime(num: i32) -> bool {
    let upper_bound = (num as f64).sqrt() as i32;
    for i in 2..=upper_bound {
        if num % i == 0 {
            return false;
        }
    }

    true
}

#[aoc(day23, part2)]
pub fn part2(_: &[Instruction]) -> usize {
    (106_500..=123_500)
        .step_by(17)
        .filter(|it| !is_prime(*it))
        .count()
}
