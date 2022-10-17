use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Direction {
    #[display("n")]
    North,

    #[display("ne")]
    NorthEast,

    #[display("se")]
    SouthEast,

    #[display("s")]
    South,

    #[display("sw")]
    SouthWest,

    #[display("nw")]
    NorthWest,
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    q: isize,
    r: isize,
    s: isize,
}

impl Cell {
    fn new(q: isize, r: isize, s: isize) -> Self {
        Self { q, r, s }
    }

    fn distance(&self, other: &Cell) -> usize {
        (self.q.abs_diff(other.q) + self.r.abs_diff(other.r) + self.s.abs_diff(other.s)) / 2
    }

    fn walk(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self::new(self.q, self.r - 1, self.s + 1),
            Direction::NorthEast => Self::new(self.q + 1, self.r - 1, self.s),
            Direction::SouthEast => Self::new(self.q + 1, self.r, self.s - 1),
            Direction::South => Self::new(self.q, self.r + 1, self.s - 1),
            Direction::SouthWest => Self::new(self.q - 1, self.r + 1, self.s),
            Direction::NorthWest => Self::new(self.q - 1, self.r, self.s + 1),
        }
    }
}

#[aoc_generator(day11)]
pub fn generate(inp: &str) -> Vec<Direction> {
    inp.split(',').filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day11, part1)]
pub fn part1(inp: &[Direction]) -> usize {
    let start_cell = Cell::new(0, 0, 0);

    let mut cur_cell = start_cell;
    for dir in inp {
        cur_cell = cur_cell.walk(*dir);
    }

    start_cell.distance(&cur_cell)
}

#[aoc(day11, part2)]
pub fn part2(inp: &[Direction]) -> Option<usize> {
    let start_cell = Cell::new(0, 0, 0);

    inp.iter()
        .scan(start_cell, |acc, &dir| {
            *acc = acc.walk(dir);
            Some(*acc)
        })
        .map(|it| it.distance(&start_cell))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_p1() {
        assert_eq!(
            part1(&[
                Direction::NorthEast,
                Direction::NorthEast,
                Direction::NorthEast
            ]),
            3
        );

        assert_eq!(
            part1(&[
                Direction::NorthEast,
                Direction::NorthEast,
                Direction::SouthWest,
                Direction::SouthWest
            ]),
            0
        );

        assert_eq!(
            part1(&[
                Direction::NorthEast,
                Direction::NorthEast,
                Direction::South,
                Direction::South
            ]),
            2
        );

        assert_eq!(
            part1(&[
                Direction::SouthEast,
                Direction::SouthWest,
                Direction::SouthEast,
                Direction::SouthWest,
                Direction::SouthWest,
            ]),
            3
        );
    }
}
