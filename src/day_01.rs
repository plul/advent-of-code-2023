//! https://adventofcode.com/2023/day/1

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let input = parser::parse(input);
    ""
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let input = parser::parse(input);
    ""
}

mod parser {
    use super::*;
    use crate::my_nom_prelude::*;

    pub(super) fn parse(s: &str) -> Vec<()> {
        all_consuming(terminated(separated_list1(line_ending, main_parser), multispace0))(s)
            .unwrap()
            .1
    }

    fn main_parser(s: &str) -> IResult<&str, ()> {
        Ok((s, ()))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE).to_string(), "142");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE).to_string(), "");
}
