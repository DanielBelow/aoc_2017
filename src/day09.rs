use aoc_runner_derive::aoc;
use std::str::Chars;

fn skip_garbage(chrs: &mut Chars<'_>, skipped_chars: &mut u64) {
    let mut ignore_next = false;
    for c in chrs.by_ref() {
        if ignore_next {
            ignore_next = false;
            continue;
        }

        if c == '>' {
            return;
        }

        ignore_next = c == '!';
        *skipped_chars += u64::from(!ignore_next);
    }
}

#[aoc(day09, part1)]
pub fn part1(inp: &str) -> u64 {
    let mut cur_depth = 0;
    let mut result = 0;
    let mut chrs = inp.chars();

    let mut skipped_chars = 0;

    while let Some(next) = chrs.next() {
        match next {
            '<' => skip_garbage(&mut chrs, &mut skipped_chars),
            '{' => cur_depth += 1,
            '}' => {
                result += cur_depth;
                cur_depth -= 1;
            }
            _ => continue,
        };
    }

    result
}

#[aoc(day09, part2)]
pub fn part2(inp: &str) -> u64 {
    let mut chrs = inp.chars();

    let mut result = 0;

    while let Some(next) = chrs.next() {
        if next == '<' {
            skip_garbage(&mut chrs, &mut result);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_garbage() {
        for (garbage, num_skipped) in [
            ("<>", 0),
            ("<random characters>", 17),
            ("<<<<>", 3),
            ("<{!>}>", 2),
            ("<!!>", 0),
            ("<!!!>>", 0),
            ("<{o\"i!a,<{i<a>", 10),
        ] {
            let mut chrs = garbage.chars();
            let _ = chrs.next(); // drop leading '<'
            let mut skipped = 0;
            skip_garbage(&mut chrs, &mut skipped);

            assert!(chrs.next().is_none());
            assert_eq!(num_skipped, skipped);
        }
    }

    #[test]
    fn test_sample_p1() {
        let inputs = [
            ("{}", 1),
            ("{{{}}}", 6),
            ("{{}, {}}", 5),
            ("{{{},{},{{}}}}", 16),
            ("{<a>,<a>,<a>,<a>}", 1),
            ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
            ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
            ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
        ];

        for (inp, expected) in inputs {
            let res = part1(inp);
            assert_eq!(res, expected);
        }
    }
}
