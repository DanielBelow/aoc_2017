use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn is_same_cardinal(self, other: Self) -> bool {
        match self {
            Self::Up | Self::Down => other == Self::Up || other == Self::Down,
            Self::Right | Self::Left => other == Self::Right || other == Self::Left,
        }
    }
}

#[aoc_generator(day19)]
pub fn generate(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|it| it.chars().collect()).collect()
}

fn get_new_directions(dir: Direction) -> Vec<Direction> {
    let mut result = vec![
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    result.retain(|it| !it.is_same_cardinal(dir));

    result
}

const fn get_next_coords(row: usize, col: usize, dir: Direction) -> Option<(usize, usize)> {
    match dir {
        Direction::Up => {
            if row == 0 {
                None
            } else {
                Some((row - 1, col))
            }
        }
        Direction::Right => Some((row, col + 1)),
        Direction::Down => Some((row + 1, col)),
        Direction::Left => {
            if col == 0 {
                None
            } else {
                Some((row, col - 1))
            }
        }
    }
}

fn walk_path(inp: &[Vec<char>]) -> Option<(String, usize)> {
    let mut row = 0;
    let (mut col, _) = inp[row].iter().find_position(|it| **it == '|')?;

    let mut dir = Direction::Down;

    let mut seen_letters = String::new();
    let mut num_steps = 1;

    loop {
        (row, col) = get_next_coords(row, col, dir).expect("This has to be safe");

        let c = inp[row][col];
        match c {
            c if c.is_ascii_alphabetic() => seen_letters.push(c),
            ' ' => break,
            '+' => {
                // update direction
                let try_to_turn = match get_next_coords(row, col, dir) {
                    None => true,
                    Some((nr, nc)) => nr >= inp.len() || nc >= inp[nr].len() || inp[nr][nc] == ' ',
                };

                if try_to_turn {
                    // We can't keep going, try turning
                    for new_dir in get_new_directions(dir) {
                        if let Some((nr, nc)) = get_next_coords(row, col, new_dir) {
                            if nr < inp.len() && nc < inp[nr].len() && inp[nr][nc] != ' ' {
                                dir = new_dir;
                                break;
                            }
                        }
                    }
                }
            }
            _ => {}
        };

        num_steps += 1;
    }

    Some((seen_letters, num_steps))
}

#[aoc(day19, part1)]
pub fn part1(inp: &[Vec<char>]) -> Option<String> {
    walk_path(inp).map(|(letters, _)| letters)
}

#[aoc(day19, part2)]
pub fn part2(inp: &[Vec<char>]) -> Option<usize> {
    walk_path(inp).map(|(_, path_len)| path_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str =
        "     |         \n     |  +--+   \n     A  |  C   \n F---|----E|--+\n     |  |  |  D\n     +B-+  +--+";

    #[test]
    fn test_sample_p1() {
        let data = generate(TEST_DATA);
        let result = part1(&data);
        assert_eq!(Some("ABCDEF".to_string()), result);
    }

    #[test]
    fn test_sample_p2() {
        let data = generate(TEST_DATA);
        let result = part2(&data);
        assert_eq!(Some(38), result);
    }
}
