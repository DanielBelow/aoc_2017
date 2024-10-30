use aoc_runner_derive::{aoc, aoc_generator};
use num_complex::Complex;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

const fn next_state(state: State, part_one: bool) -> State {
    match state {
        State::Clean => {
            if part_one {
                State::Infected
            } else {
                State::Weakened
            }
        }
        State::Weakened => State::Infected,
        State::Infected => {
            if part_one {
                State::Clean
            } else {
                State::Flagged
            }
        }
        State::Flagged => State::Clean,
    }
}

#[derive(Clone)]
pub struct Grid {
    map: VecDeque<VecDeque<State>>,
}

impl Grid {
    fn grow_top(&mut self) {
        let width = self.map[0].len();
        let mut new_top_row = VecDeque::new();
        new_top_row.resize(width, State::Clean);
        self.map.push_front(new_top_row);
    }

    fn grow_bottom(&mut self) {
        let width = self.map[0].len();
        let mut new_bottom_row = VecDeque::new();
        new_bottom_row.resize(width, State::Clean);
        self.map.push_back(new_bottom_row);
    }

    fn grow_left(&mut self) {
        self.map
            .iter_mut()
            .for_each(|it| it.push_front(State::Clean));
    }

    fn grow_right(&mut self) {
        self.map
            .iter_mut()
            .for_each(|it| it.push_back(State::Clean));
    }
}

#[aoc_generator(day22)]
pub fn generate(inp: &str) -> Grid {
    let map = inp
        .lines()
        .map(|it| {
            it.chars()
                .map(|it| {
                    if it == '#' {
                        State::Infected
                    } else {
                        State::Clean
                    }
                })
                .collect::<VecDeque<_>>()
        })
        .collect::<VecDeque<_>>();

    Grid { map }
}

fn move_forward(pos: Complex<i64>, dir: Complex<i64>, grid: &mut Grid) -> Complex<i64> {
    match dir {
        Complex { re: -1, im: 0 } => {
            if pos.re == 0 {
                grid.grow_top();
                pos
            } else {
                pos + dir
            }
        }
        Complex { re: 0, im: 1 } => {
            #[allow(clippy::cast_possible_wrap)]
            if pos.im == (grid.map[0].len() - 1) as i64 {
                grid.grow_right();
            }

            pos + dir
        }
        Complex { re: 1, im: 0 } => {
            #[allow(clippy::cast_possible_wrap)]
            if pos.re == (grid.map.len() - 1) as i64 {
                grid.grow_bottom();
            }

            pos + dir
        }
        Complex { re: 0, im: -1 } => {
            if pos.im == 0 {
                grid.grow_left();
                pos
            } else {
                pos + dir
            }
        }
        _ => unreachable!(),
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn run_bursts<FState>(grid: &Grid, num_steps: usize, next_state_fn: FState) -> usize
where
    FState: Fn(State) -> State,
{
    let (car_x, car_y) = (grid.map[0].len() / 2, grid.map.len() / 2);

    #[allow(clippy::cast_possible_wrap)]
    let mut pos = Complex::new(car_x as i64, car_y as i64);
    let mut dir = Complex::new(-1, 0);

    let mut grid = grid.clone();

    let mut burst_infections = 0;

    for _ in 0..num_steps {
        let cur_node = &mut grid.map[pos.re as usize][pos.im as usize];

        dir = match *cur_node {
            State::Clean => dir * Complex::i(),
            State::Weakened => dir,
            State::Infected => dir * Complex::new(0, -1),
            State::Flagged => dir * Complex::i().powi(2),
        };

        *cur_node = next_state_fn(*cur_node);
        burst_infections += usize::from(matches!(cur_node, State::Infected));

        pos = move_forward(pos, dir, &mut grid);
    }

    burst_infections
}

#[aoc(day22, part1)]
pub fn part1(grid: &Grid) -> usize {
    run_bursts(grid, 10_000, |it| next_state(it, true))
}

#[aoc(day22, part2)]
pub fn part2(grid: &Grid) -> usize {
    run_bursts(grid, 10_000_000, |it| next_state(it, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inp = "..#\n\
                          #..\n\
                          ...";
        let data = generate(inp);
        let result = run_bursts(&data, 10_000, |state| next_state(state, true));
        assert_eq!(result, 5587);
    }

    #[test]
    fn test_sample_p2() {
        let inp = "..#\n\
                          #..\n\
                          ...";
        let data = generate(inp);
        let result = run_bursts(&data, 100, |state| next_state(state, false));
        assert_eq!(result, 26);

        let result = run_bursts(&data, 10_000_000, |state| next_state(state, false));
        assert_eq!(result, 2_511_944);
    }
}
