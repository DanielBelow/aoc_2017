use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn is_valid_phrase(word: &str, transform: fn(&str) -> String) -> bool {
    let words = word.split_ascii_whitespace().map(transform).collect_vec();
    let num_words = words.len();
    let unique_words = words.iter().unique().count();
    unique_words == num_words
}

#[aoc_generator(day04)]
pub fn generate(inp: &str) -> Vec<String> {
    inp.lines().map(ToString::to_string).collect_vec()
}

#[aoc(day04, part1)]
pub fn part1(inp: &[String]) -> usize {
    inp.iter()
        .filter(|it| is_valid_phrase(it, ToString::to_string))
        .count()
}

#[aoc(day04, part2)]
pub fn part2(inp: &[String]) -> usize {
    inp.iter()
        .filter(|it| is_valid_phrase(it, |word| word.chars().sorted().join("")))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples_p1() {
        let inputs = ["aa bb cc dd ee", "aa bb cc dd aa", "aa bb cc dd aaa"]
            .iter()
            .map(ToString::to_string)
            .collect_vec();

        let res = part1(&inputs);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_samples_p2() {
        let inputs = [
            "abcde fghij",
            "abcde xyz ecdab",
            "a ab abc abd abf abj",
            "iiii oiii ooii oooi oooo",
            "oiii ioii iioi iiio",
        ]
        .iter()
        .map(ToString::to_string)
        .collect_vec();

        let res = part2(&inputs);
        assert_eq!(res, 3);
    }
}
