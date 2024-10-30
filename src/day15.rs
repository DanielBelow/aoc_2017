use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("Generator {id} starts with {prev_value}")]
pub struct Generator {
    id: char,
    prev_value: usize,
}

impl Generator {
    const fn factor(&self) -> usize {
        if self.id == 'A' {
            16807
        } else {
            48271
        }
    }
}

impl IntoIterator for Generator {
    type Item = usize;
    type IntoIter = GeneratorIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        GeneratorIntoIterator {
            factor: self.factor(),
            prev_value: self.prev_value,
        }
    }
}

pub struct GeneratorIntoIterator {
    factor: usize,
    prev_value: usize,
}

impl GeneratorIntoIterator {
    const fn next_value(&self) -> usize {
        (self.prev_value * self.factor) % 2_147_483_647
    }
}

impl Iterator for GeneratorIntoIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev_value = self.next_value();
        Some(self.prev_value)
    }
}

#[aoc_generator(day15)]
pub fn generate(inp: &str) -> Option<(Generator, Generator)> {
    let mut lines = inp.lines();
    let generator_a = lines.next().and_then(|it| it.parse().ok())?;
    let generator_b = lines.next().and_then(|it| it.parse().ok())?;

    Some((generator_a, generator_b))
}

#[aoc(day15, part1)]
#[allow(clippy::cast_possible_truncation)]
pub fn part1((gen_a, gen_b): &(Generator, Generator)) -> usize {
    gen_a
        .into_iter()
        .zip(*gen_b)
        .map(|(lhs, rhs)| (lhs as u16, rhs as u16))
        .take(40_000_000)
        .filter(|(lhs, rhs)| *lhs == *rhs)
        .count()
}

#[aoc(day15, part2)]
#[allow(clippy::cast_possible_truncation)]
pub fn part2((gen_a, gen_b): &(Generator, Generator)) -> usize {
    gen_a
        .into_iter()
        .filter(|it| *it % 4 == 0)
        .zip(gen_b.into_iter().filter(|it| *it % 8 == 0))
        .map(|(lhs, rhs)| (lhs as u16, rhs as u16))
        .take(5_000_000)
        .filter(|(lhs, rhs)| *lhs == *rhs)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_generator_iterator() {
        let gen_a = Generator {
            id: 'A',
            prev_value: 65,
        };

        let values = gen_a.into_iter().take(5).collect_vec();
        assert_eq!(
            vec![
                1_092_455,
                1_181_022_009,
                245_556_042,
                1_744_312_007,
                1_352_636_452
            ],
            values
        );
    }

    #[test]
    fn test_sample_p1() {
        let gen_a = Generator {
            id: 'A',
            prev_value: 65,
        };
        let gen_b = Generator {
            id: 'B',
            prev_value: 8921,
        };

        let result = part1(&(gen_a, gen_b));
        assert_eq!(588, result);
    }

    #[test]
    fn test_sample_p2() {
        let gen_a = Generator {
            id: 'A',
            prev_value: 65,
        };
        let gen_b = Generator {
            id: 'B',
            prev_value: 8921,
        };

        let result = part2(&(gen_a, gen_b));
        assert_eq!(309, result);
    }
}
