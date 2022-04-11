use aoc_runner_derive::aoc;

fn next_letter(inp: &str, next_index: usize) -> char {
    inp.chars()
        .nth(next_index)
        .expect("Expected wrapping index")
}

fn calculate_checksum(inp: &str, part_2: bool) -> u32 {
    let num_chars = inp.len();
    let char_offset = if part_2 { num_chars / 2 } else { 1 };

    inp.chars().enumerate().fold(0, |acc, (idx, cur)| {
        let next = next_letter(inp, (idx + char_offset) % num_chars);
        acc + if next == cur {
            cur.to_digit(10).expect("Only digits in input")
        } else {
            0
        }
    })
}

#[aoc(day01, part1)]
pub fn part1(inp: &str) -> u32 {
    calculate_checksum(inp, false)
}

#[aoc(day01, part2)]
pub fn part2(inp: &str) -> u32 {
    calculate_checksum(inp, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_p1() {
        let inputs = [("1122", 3), ("1111", 4), ("1234", 0), ("91212129", 9)];

        for (inp, expected) in inputs {
            let res = part1(inp);
            assert_eq!(expected, res);
        }
    }

    #[test]
    fn test_samples_p2() {
        let inputs = [
            ("1212", 6),
            ("1221", 0),
            ("123425", 4),
            ("123123", 12),
            ("12131415", 4),
        ];

        for (inp, expected) in inputs {
            let res = part2(inp);
            assert_eq!(expected, res);
        }
    }
}
