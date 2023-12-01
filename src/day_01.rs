//! https://adventofcode.com/2023/day/1

use type_toppings::ResultExt as _;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    parser::parse_part_1(input)
        .into_iter()
        .map(|DigitPair { first_digit, last_digit }| format!("{}{}", first_digit, last_digit))
        .map(|s| s.parse::<usize>().unwrap_or_report())
        .sum::<usize>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    parser::parse_part_2(input)
        .into_iter()
        .map(|DigitPair { first_digit, last_digit }| format!("{}{}", first_digit, last_digit))
        .map(|s| s.parse::<usize>().unwrap_or_report())
        .sum::<usize>()
}

struct DigitPair {
    first_digit: char,
    last_digit: char,
}

mod parser {
    use super::*;

    pub(super) fn parse_part_1(s: &str) -> Vec<DigitPair> {
        s.lines().map(|line| -> DigitPair { parse_line_part_1(line) }).collect()
    }

    pub(super) fn parse_part_2(s: &str) -> Vec<DigitPair> {
        s.lines().map(|line| -> DigitPair { parse_line_part_2(line) }).collect()
    }

    fn parse_line_part_1(s: &str) -> DigitPair {
        let mut i = 0;
        let first_digit = loop {
            assert!(s.len() >= i, "first digit search out of bounds");
            let mid = i;
            let (_, s) = s.split_at(mid);
            match s {
                _ if s.starts_with("0") => break '0',
                _ if s.starts_with("1") => break '1',
                _ if s.starts_with("2") => break '2',
                _ if s.starts_with("3") => break '3',
                _ if s.starts_with("4") => break '4',
                _ if s.starts_with("5") => break '5',
                _ if s.starts_with("6") => break '6',
                _ if s.starts_with("7") => break '7',
                _ if s.starts_with("8") => break '8',
                _ if s.starts_with("9") => break '9',
                _ => {}
            }
            i += 1;
        };

        let mut i = 0;
        let last_digit = loop {
            assert!(s.len() >= i, "last digit search out of bounds");
            let mid = s.len() - i;
            let (s, _) = s.split_at(mid);
            match s {
                _ if s.ends_with("0") => break '0',
                _ if s.ends_with("1") => break '1',
                _ if s.ends_with("2") => break '2',
                _ if s.ends_with("3") => break '3',
                _ if s.ends_with("4") => break '4',
                _ if s.ends_with("5") => break '5',
                _ if s.ends_with("6") => break '6',
                _ if s.ends_with("7") => break '7',
                _ if s.ends_with("8") => break '8',
                _ if s.ends_with("9") => break '9',
                _ => {}
            }
            i += 1;
        };

        DigitPair { first_digit, last_digit }
    }

    fn parse_line_part_2(s: &str) -> DigitPair {
        let mut i = 0;
        let first_digit = loop {
            assert!(s.len() >= i, "first digit search out of bounds");
            let mid = i;
            let (_, s) = s.split_at(mid);
            match s {
                _ if s.starts_with("0") => break '0',
                _ if s.starts_with("1") => break '1',
                _ if s.starts_with("2") => break '2',
                _ if s.starts_with("3") => break '3',
                _ if s.starts_with("4") => break '4',
                _ if s.starts_with("5") => break '5',
                _ if s.starts_with("6") => break '6',
                _ if s.starts_with("7") => break '7',
                _ if s.starts_with("8") => break '8',
                _ if s.starts_with("9") => break '9',
                _ if s.starts_with("zero") => break '0',
                _ if s.starts_with("one") => break '1',
                _ if s.starts_with("two") => break '2',
                _ if s.starts_with("three") => break '3',
                _ if s.starts_with("four") => break '4',
                _ if s.starts_with("five") => break '5',
                _ if s.starts_with("six") => break '6',
                _ if s.starts_with("seven") => break '7',
                _ if s.starts_with("eight") => break '8',
                _ if s.starts_with("nine") => break '9',
                _ => {}
            }
            i += 1;
        };

        let mut i = 0;
        let last_digit = loop {
            assert!(s.len() >= i, "last digit search out of bounds");
            let mid = s.len() - i;
            let (s, _) = s.split_at(mid);
            match s {
                _ if s.ends_with("0") => break '0',
                _ if s.ends_with("1") => break '1',
                _ if s.ends_with("2") => break '2',
                _ if s.ends_with("3") => break '3',
                _ if s.ends_with("4") => break '4',
                _ if s.ends_with("5") => break '5',
                _ if s.ends_with("6") => break '6',
                _ if s.ends_with("7") => break '7',
                _ if s.ends_with("8") => break '8',
                _ if s.ends_with("9") => break '9',
                _ if s.ends_with("zero") => break '0',
                _ if s.ends_with("one") => break '1',
                _ if s.ends_with("two") => break '2',
                _ if s.ends_with("three") => break '3',
                _ if s.ends_with("four") => break '4',
                _ if s.ends_with("five") => break '5',
                _ if s.ends_with("six") => break '6',
                _ if s.ends_with("seven") => break '7',
                _ if s.ends_with("eight") => break '8',
                _ if s.ends_with("nine") => break '9',
                _ => {}
            }
            i += 1;
        };

        DigitPair { first_digit, last_digit }
    }
}

#[cfg(test)]
static EXAMPLE_PART_1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

#[cfg(test)]
static EXAMPLE_PART_2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE_PART_1).to_string(), "142");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE_PART_2).to_string(), "281");
}
