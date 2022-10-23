use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn reverse(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn next_state(state: State, part_one: bool) -> State {
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

fn move_forward(x: usize, y: usize, dir: Direction, grid: &mut Grid) -> (usize, usize) {
    match dir {
        Direction::Up => {
            if x == 0 {
                grid.grow_top();
                (x, y)
            } else {
                (x - 1, y)
            }
        }
        Direction::Right => {
            if y == grid.map[0].len() - 1 {
                grid.grow_right();
            }

            (x, y + 1)
        }
        Direction::Down => {
            if x == grid.map.len() - 1 {
                grid.grow_bottom();
            }

            (x + 1, y)
        }
        Direction::Left => {
            if y == 0 {
                grid.grow_left();
                (x, y)
            } else {
                (x, y - 1)
            }
        }
    }
}

fn run_bursts<FState>(grid: &mut Grid, num_steps: usize, next_state_fn: FState) -> usize
where
    FState: Fn(State) -> State,
{
    let (mut car_x, mut car_y) = (grid.map.len() / 2, grid.map[0].len() / 2);
    let mut car_dir = Direction::Up;

    let mut grid = grid.clone();

    let mut burst_infections = 0;

    for _ in 0..num_steps {
        let cur_node = &mut grid.map[car_x][car_y];

        car_dir = match *cur_node {
            State::Clean => car_dir.turn_left(),
            State::Weakened => car_dir,
            State::Infected => car_dir.turn_right(),
            State::Flagged => car_dir.reverse(),
        };

        *cur_node = next_state_fn(*cur_node);
        if matches!(cur_node, State::Infected) {
            burst_infections += 1;
        }

        (car_x, car_y) = move_forward(car_x, car_y, car_dir, &mut grid);
    }

    burst_infections
}

#[aoc(day22, part1)]
pub fn part1(grid: &Grid) -> usize {
    run_bursts(&mut grid.clone(), 10_000, |it| next_state(it, true))
}

#[aoc(day22, part2)]
pub fn part2(grid: &Grid) -> usize {
    run_bursts(&mut grid.clone(), 10_000_000, |it| next_state(it, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inp = "..#\n\
                          #..\n\
                          ...";
        let mut data = generate(inp);
        let result = run_bursts(&mut data, 10_000, |state| next_state(state, true));
        assert_eq!(result, 5587);
    }

    #[test]
    fn test_sample_p2() {
        let inp = "..#\n\
                          #..\n\
                          ...";
        let mut data = generate(inp);
        let result = run_bursts(&mut data.clone(), 100, |state| next_state(state, false));
        assert_eq!(result, 26);

        let result = run_bursts(&mut data, 10_000_000, |state| next_state(state, false));
        assert_eq!(result, 2_511_944);
    }
}
