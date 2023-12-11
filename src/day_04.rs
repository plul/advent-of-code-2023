//! https://adventofcode.com/2023/day/4

use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    parser::parse(input)
        .into_iter()
        .map(Line::matches)
        .map(|matches| {
            let mut points = 0;
            if matches > 0 {
                points = 1;
            }
            for _ in 1..matches {
                points *= 2;
            }
            points
        })
        .sum::<usize>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut sum = 0;
    let mut copies = VecDeque::new();
    for line in parser::parse(input) {
        let instances = 1 + copies.pop_front().unwrap_or(0);
        sum += instances;
        let matches = line.matches();
        while copies.len() < matches {
            copies.push_back(0);
        }
        for entry in copies.range_mut(0..matches) {
            *entry += instances;
        }
    }
    sum
}

struct Line {
    winning_numbers: Vec<u64>,
    numbers_you_have: Vec<u64>,
}
impl Line {
    fn matches(self) -> usize {
        let winning_numbers: HashSet<u64> = HashSet::from_iter(self.winning_numbers);
        let numbers_you_have: HashSet<u64> = HashSet::from_iter(self.numbers_you_have);
        let intersection = winning_numbers.intersection(&numbers_you_have);
        intersection.count()
    }
}

mod parser {
    use super::*;
    use crate::my_nom_prelude::*;
    use type_toppings::ResultExt as _;

    pub(super) fn parse(s: &str) -> Vec<Line> {
        s.lines().map(|line| all_consuming(parse_line)(line).expect_or_report(line).1).collect()
    }

    fn parse_line(s: &str) -> IResult<&str, Line> {
        let (s, _) = tag("Card")(s)?;
        let (s, _) = multispace1(s)?;
        let (s, _) = u64(s)?;
        let (s, _) = tag(":")(s)?;
        let (s, _) = multispace1(s)?;
        let (s, winning_numbers) = separated_list1(multispace1, u64)(s)?;
        let (s, _) = delimited(multispace1, tag("|"), multispace1)(s)?;
        let (s, numbers_you_have) = separated_list1(multispace1, u64)(s)?;
        let line = Line {
            winning_numbers,
            numbers_you_have,
        };
        Ok((s, line))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE).to_string(), "13");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE).to_string(), "30");
}
