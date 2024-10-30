use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, PartialEq, Eq, Copy, Clone)]
#[display("{depth}: {range}")]
pub struct Layer {
    depth: usize,
    range: usize,
    #[from_str(default)]
    cur_pos: usize,

    #[from_str(default)]
    down: bool,
}

impl Layer {
    const fn severity(&self) -> usize {
        self.depth * self.range
    }
}

#[aoc_generator(day13)]
pub fn generate(inp: &str) -> Vec<Layer> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

fn update_scanners(map: &mut HashMap<usize, Layer>) {
    for (_, val) in map.iter_mut() {
        val.cur_pos = if val.cur_pos == val.range - 1 {
            val.down = true;
            val.cur_pos - 1
        } else if val.cur_pos == 0 {
            val.down = false;
            val.cur_pos + 1
        } else if val.down {
            val.cur_pos - 1
        } else {
            val.cur_pos + 1
        };
    }
}

fn calculate_severity(
    max_depth: usize,
    layers: &mut HashMap<usize, Layer>,
    stop_if_caught: bool,
) -> Option<usize> {
    let mut total_severity = 0;
    let mut cur_depth = 0;

    loop {
        if cur_depth > max_depth {
            break;
        }

        if let Some(hit_scanner) = layers.get(&cur_depth) {
            if hit_scanner.cur_pos == 0 {
                total_severity += hit_scanner.severity();
                if stop_if_caught {
                    return Some(0);
                }
            }
        }

        update_scanners(layers);

        cur_depth += 1;
    }

    if total_severity == 0 {
        None
    } else {
        Some(total_severity)
    }
}

#[aoc(day13, part1)]
pub fn part1(data: &[Layer]) -> Option<usize> {
    let max_depth = data.iter().last()?.depth;
    let mut layers_map = data
        .iter()
        .map(|it| (it.depth, *it))
        .collect::<HashMap<_, _>>();

    calculate_severity(max_depth, &mut layers_map, false)
}

#[aoc(day13, part2)]
pub fn part2(data: &[Layer]) -> Option<usize> {
    let max_depth = data.iter().last()?.depth;
    let layers_map = data
        .iter()
        .map(|it| (it.depth, *it))
        .collect::<HashMap<_, _>>();

    let mut delayed_state = layers_map;
    update_scanners(&mut delayed_state);

    let mut delay = 1;
    while calculate_severity(max_depth, &mut delayed_state.clone(), true).is_some() {
        update_scanners(&mut delayed_state);
        delay += 1;
    }

    Some(delay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let test_data = "0: 3\n\
                                1: 2\n\
                                4: 4\n\
                                6: 4";
        let data = generate(test_data);
        let result = part1(&data);
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_sample_p2() {
        let test_data = "0: 3\n\
                                1: 2\n\
                                4: 4\n\
                                6: 4";
        let data = generate(test_data);
        let result = part2(&data);
        assert_eq!(result, Some(10));
    }
}
