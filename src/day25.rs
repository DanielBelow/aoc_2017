use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;
use Direction::{Left, Right};

type StateTransition = fn(&mut VecDeque<u8>, &mut usize) -> usize;

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub struct State {
    if_zero: StateTransition,
    if_one: StateTransition,
}

#[derive(Clone)]
pub struct StateMachine {
    cur_state: usize,
    states: Vec<State>,

    cursor: usize,
    tape: VecDeque<u8>,
}

impl StateMachine {
    pub fn checksum(&self) -> usize {
        self.tape.iter().filter(|it| **it == 1).count()
    }

    pub fn step(&mut self) {
        let state = &self.states[self.cur_state];

        self.cur_state = match self.tape[self.cursor] {
            0 => (state.if_zero)(&mut self.tape, &mut self.cursor),
            1 => (state.if_one)(&mut self.tape, &mut self.cursor),
            _ => panic!("Invalid value on tape!"),
        };
    }
}

fn do_work(
    tape: &mut VecDeque<u8>,
    cursor: &mut usize,
    to_write: u8,
    update: Direction,
    next_state: usize,
) -> usize {
    // Write the value
    tape[*cursor] = to_write;

    // Move one slot
    match update {
        Left => tape.push_front(0),
        Right => {
            *cursor += 1;
            tape.push_back(0);
        }
    };

    // Continue with state
    next_state
}

#[aoc_generator(day25)]
pub fn generate(_: &str) -> StateMachine {
    let mut tape = VecDeque::with_capacity(4096);
    tape.push_back(0);

    let mut state_machine = StateMachine {
        cur_state: 0,
        states: Vec::with_capacity(6),
        cursor: 0,
        tape,
    };

    let a_state = State {
        if_zero: |tape, cursor| do_work(tape, cursor, 1, Right, 1),
        if_one: |tape, cursor| do_work(tape, cursor, 0, Right, 2),
    };

    let b_state = State {
        if_zero: |tape, cursor| do_work(tape, cursor, 0, Left, 0),
        if_one: |tape, cursor| do_work(tape, cursor, 0, Right, 3),
    };

    let c_state = State {
        if_zero: |tape, cursor| do_work(tape, cursor, 1, Right, 3),
        if_one: |tape, cursor| do_work(tape, cursor, 1, Right, 0),
    };

    let d_state = State {
        if_zero: |tape, cursor| do_work(tape, cursor, 1, Left, 4),
        if_one: |tape, cursor| do_work(tape, cursor, 0, Left, 3),
    };

    let e_state = State {
        if_zero: |tape, cursor| do_work(tape, cursor, 1, Right, 5),
        if_one: |tape, cursor| do_work(tape, cursor, 1, Left, 1),
    };

    let f_state = State {
        if_zero: |tape, cursor| do_work(tape, cursor, 1, Right, 0),
        if_one: |tape, cursor| do_work(tape, cursor, 1, Right, 4),
    };

    state_machine.states.push(a_state);
    state_machine.states.push(b_state);
    state_machine.states.push(c_state);
    state_machine.states.push(d_state);
    state_machine.states.push(e_state);
    state_machine.states.push(f_state);

    state_machine
}

#[aoc(day25, part1)]
pub fn part1(state_machine: &StateMachine) -> usize {
    const NUM_STEPS: usize = 12_368_930;

    let mut state_machine = state_machine.clone();

    for _ in 0..NUM_STEPS {
        state_machine.step();
    }

    state_machine.checksum()
}
