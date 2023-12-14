//! https://adventofcode.com/2023/day/n

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let input = parser::parse(input);
    ""
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    ""
}

struct Line {}

mod parser {
    use super::*;
    use crate::my_nom_prelude::*;
    use type_toppings::ResultExt as _;

    pub(super) fn parse(s: &str) -> Vec<Line> {
        s.lines().map(|line| all_consuming(parse_line)(line).expect_or_report(line).1).collect()
    }

    fn parse_line(s: &str) -> IResult<&str, Line> {
        Ok((s, ()))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\

";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE).to_string(), "");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE).to_string(), "");
}
