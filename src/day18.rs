use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use std::collections::{HashMap, VecDeque};
use ExecutionResult::{Continue, Terminated};

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("format")]
pub enum Value {
    #[from_str(regex = "(?P<0>[A-Za-z])")]
    Register(char),

    #[from_str(regex = "(?P<0>[-0-9]+)")]
    Immediate(i64),
}

impl Value {
    fn evaluate(self, regs: &mut HashMap<char, i64>) -> i64 {
        match self {
            Value::Immediate(val) => val,
            Value::Register(r) => *regs.entry(r).or_default(),
        }
    }
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
pub enum Instruction {
    #[display("snd {0}")]
    Snd(Value),

    #[display("set {0} {1}")]
    Set(char, Value),

    #[display("add {0} {1}")]
    Add(char, Value),

    #[display("mul {0} {1}")]
    Mul(char, Value),

    #[display("mod {0} {1}")]
    Mod(char, Value),

    #[display("rcv {0}")]
    Rcv(Value),

    #[display("jgz {0} {1}")]
    JumpGZ(Value, Value),
}

#[aoc_generator(day18)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

#[derive(Default)]
struct ExecutionContext<'a> {
    pc: isize,
    insts: &'a [Instruction],
    last_sound: Option<i64>,
    regs: HashMap<char, i64>,

    pipe: VecDeque<i64>,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum ExecutionResult {
    Continue(bool),
    Terminated,
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl<'a> ExecutionContext<'a> {
    fn is_waiting_on_input(&self) -> bool {
        assert!(self.pc >= 0 && ((self.pc as usize) < self.insts.len()));
        matches!(self.insts[self.pc as usize], Instruction::Rcv(_)) && self.pipe.is_empty()
    }

    fn execute(&mut self, other: Option<&mut ExecutionContext<'a>>) -> ExecutionResult {
        assert!(self.pc >= 0 && ((self.pc as usize) < self.insts.len()));

        let cur_pc = self.pc as usize;

        let mut sent_data = false;

        match self.insts[cur_pc] {
            Instruction::Snd(val) => {
                let val = val.evaluate(&mut self.regs);
                if let Some(other_ctx) = other {
                    sent_data = true;
                    other_ctx.pipe.push_back(val);
                } else {
                    self.last_sound = Some(val);
                }

                self.pc += 1;
            }
            Instruction::Set(reg, val) => {
                let evaluated = val.evaluate(&mut self.regs);
                *self.regs.entry(reg).or_default() = evaluated;
                self.pc += 1;
            }
            Instruction::Add(reg, val) => {
                let evaluated = val.evaluate(&mut self.regs);
                *self.regs.entry(reg).or_default() += evaluated;
                self.pc += 1;
            }
            Instruction::Mul(reg, val) => {
                let evaluated = val.evaluate(&mut self.regs);
                *self.regs.entry(reg).or_default() *= evaluated;
                self.pc += 1;
            }
            Instruction::Mod(reg, val) => {
                let evaluated = val.evaluate(&mut self.regs);
                *self.regs.entry(reg).or_default() %= evaluated;
                self.pc += 1;
            }
            Instruction::Rcv(val) => {
                if let Some(elem) = self.pipe.pop_front() {
                    assert!(matches!(val, Value::Register(_)));

                    if let Value::Register(r) = val {
                        *self.regs.entry(r).or_default() = elem;
                        self.pc += 1;
                    }
                } else if other.is_none() {
                    if val.evaluate(&mut self.regs) == 0 {
                        self.pc += 1;
                    } else {
                        return Terminated;
                    }
                }
            }
            Instruction::JumpGZ(cond, offset) => {
                let cond = cond.evaluate(&mut self.regs);
                self.pc += if cond > 0 {
                    offset.evaluate(&mut self.regs) as isize
                } else {
                    1
                };
            }
        };

        if self.pc < 0 || (self.pc as usize >= self.insts.len()) {
            Terminated
        } else {
            Continue(sent_data)
        }
    }
}

#[aoc(day18, part1)]
pub fn part1(insts: &[Instruction]) -> Option<i64> {
    let mut context = ExecutionContext {
        insts,
        ..Default::default()
    };

    loop {
        match context.execute(None) {
            Continue(_) => {}
            Terminated => return context.last_sound,
        }
    }
}

#[aoc(day18, part2)]
pub fn part2(insts: &[Instruction]) -> usize {
    let mut context_a = ExecutionContext {
        insts,
        ..Default::default()
    };
    context_a.regs.insert('p', 0);

    let mut context_b = ExecutionContext {
        insts,
        ..Default::default()
    };
    context_b.regs.insert('p', 1);

    let mut num_sends = 0;

    let mut state_a = Continue(false);
    let mut state_b = Continue(false);

    loop {
        if !matches!(state_a, Terminated) {
            state_a = context_a.execute(Some(&mut context_b));
        }

        if !matches!(state_b, Terminated) {
            state_b = context_b.execute(Some(&mut context_a));
        }

        if let Continue(sent) = state_b {
            num_sends += usize::from(sent);
        }

        if state_a == Terminated && state_b == Terminated
            || (context_a.is_waiting_on_input() && context_b.is_waiting_on_input())
        {
            return num_sends;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let test_data = "set a 1\n\
                                add a 2\n\
                                mul a a\n\
                                mod a 5\n\
                                snd a\n\
                                set a 0\n\
                                rcv a\n\
                                jgz a -1\n\
                                set a 1\n\
                                jgz a -2";

        let data = generate(test_data);
        let result = part1(&data);
        assert_eq!(Some(4), result);
    }

    #[test]
    fn test_sample_p2_no_deadlock() {
        let test_data = "snd 1\n\
                                snd 2\n\
                                snd p\n\
                                rcv a\n\
                                rcv b\n\
                                rcv c\n\
                                rcv d";

        let data = generate(test_data);
        let result = part2(&data);
        assert_eq!(3, result);
    }
}
