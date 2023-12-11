//! https://adventofcode.com/2023/day/2

use std::cmp::max;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    parser::parse(input)
        .into_iter()
        .filter(|game| !game.reveals.iter().any(|reveal| reveal.red > 12 || reveal.green > 13 || reveal.blue > 14))
        .map(|game| game.nr)
        .sum::<u64>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut sum = 0;
    for game in parser::parse(input) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for reveal in game.reveals {
            red = max(red, reveal.red);
            green = max(green, reveal.green);
            blue = max(blue, reveal.blue);
        }
        let power = red * green * blue;
        sum += power;
    }
    sum
}

struct Game {
    nr: u64,
    reveals: Vec<Reveal>,
}

#[derive(Default)]
struct Reveal {
    red: u64,
    green: u64,
    blue: u64,
}

mod parser {
    use super::*;
    use crate::my_nom_prelude::*;
    use type_toppings::ResultExt as _;

    pub(super) fn parse(s: &str) -> Vec<Game> {
        s.lines().map(|line| all_consuming(parse_line)(line).expect_or_report(line).1).collect()
    }

    fn parse_line(s: &str) -> IResult<&str, Game> {
        let (s, (_, nr, _)) = tuple((tag("Game "), u64, tag(": ")))(s)?;
        let (s, reveals) = separated_list1(tag("; "), parse_reveal)(s)?;
        Ok((s, Game { nr, reveals }))
    }

    fn parse_reveal(s: &str) -> IResult<&str, Reveal> {
        let parse_color = |t| separated_pair(u64, tag(" "), tag(t));
        let parse_colors = alt((parse_color("red"), parse_color("green"), parse_color("blue")));
        let (s, list) = separated_list1(tag(", "), parse_colors)(s)?;
        let mut reveal = Reveal::default();
        for (n, color) in list {
            match color {
                "red" => reveal.red += n,
                "green" => reveal.green += n,
                "blue" => reveal.blue += n,
                _ => unreachable!(),
            }
        }
        Ok((s, reveal))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE).to_string(), "8");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE).to_string(), "2286");
}
