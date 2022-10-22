use super::day10::part2 as knot_hash;
use aoc_runner_derive::aoc;
use itertools::{iproduct, Itertools};

fn to_binary(c: char) -> &'static str {
    assert!(c.is_ascii_hexdigit());

    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'a' => "1010",
        'b' => "1011",
        'c' => "1100",
        'd' => "1101",
        'e' => "1110",
        'f' => "1111",
        _ => unreachable!("Not a valid hex digit '{}'!", c),
    }
}

fn generate_grid(key: &str, grid: &mut [Vec<char>]) {
    for (idx, row) in grid.iter_mut().enumerate() {
        let key = format!("{key}-{idx}");
        let hash = knot_hash(&key);

        hash.chars()
            .map(to_binary)
            .flat_map(str::chars)
            .enumerate()
            .for_each(|(idx, it)| {
                row[idx] = it;
            });
    }
}

#[aoc(day14, part1)]
pub fn part1(key: &str) -> usize {
    let mut grid = vec![vec!['0'; 128]; 128];
    generate_grid(key, &mut grid);

    grid.iter().flatten().filter(|it| **it == '1').count()
}

fn get_neighbours(x: usize, y: usize, grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    if grid[x][y] == '0' {
        return vec![];
    }

    let mut neighbours = vec![];

    if x > 0 && grid[x - 1][y] == '1' {
        neighbours.push((x - 1, y));
    }

    if x < 127 && grid[x + 1][y] == '1' {
        neighbours.push((x + 1, y));
    }

    if y > 0 && grid[x][y - 1] == '1' {
        neighbours.push((x, y - 1));
    }

    if y < 127 && grid[x][y + 1] == '1' {
        neighbours.push((x, y + 1));
    }

    neighbours
}

#[aoc(day14, part2)]
pub fn part2(key: &str) -> usize {
    let mut grid = vec![vec!['0'; 128]; 128];
    generate_grid(key, &mut grid);

    let result = pathfinding::undirected::connected_components::connected_components(
        &iproduct!(0..128usize, 0..128usize).collect_vec(),
        |&(x, y)| get_neighbours(x, y, &grid),
    );

    // remove all "single" components that aren't used
    result.len() - grid.iter().flatten().filter(|it| **it == '0').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "flqrgnkx";

    #[test]
    fn test_sample_p1() {
        let result = part1(TEST_INPUT);
        assert_eq!(8108, result);
    }

    #[test]
    fn test_sample_p2() {
        let result = part2(TEST_INPUT);
        assert_eq!(1242, result);
    }
}
