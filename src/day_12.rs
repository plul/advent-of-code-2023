//! https://adventofcode.com/2023/day/12

use crate::my_nom_prelude::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    parser::parse(input)
        .into_iter()
        .map(|row| valid_arrangements(&row.springs, &row.groups))
        .sum::<u64>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    parser::parse(input)
        .into_iter()
        .map(|row| {
            let mut springs = vec![];
            let mut groups = vec![];
            for _ in 0..5 {
                springs.extend(&row.springs);
                groups.extend(&row.groups);
            }
            Row { springs, groups }
        })
        .inspect(|row| {
            let s: String = row.springs.iter().map(ToString::to_string).collect();
            println!("{}", s);
        })
        .map(|row| valid_arrangements(&row.springs, &row.groups))
        .sum::<u64>()
}

/// Contiguous group of damaged springs
type ContiguousGroup = u64;

#[derive(Debug, Copy, Clone, derive_more::Display)]
enum Spring {
    #[display(fmt = ".")]
    Operational,

    #[display(fmt = "#")]
    Damaged,

    #[display(fmt = "?")]
    Unknown,
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    groups: Vec<ContiguousGroup>,
}

/// Returns the number of different (valid) arrangements
fn valid_arrangements(springs: &[Spring], groups: &[ContiguousGroup]) -> u64 {
    if can_be_ruled_out(springs, groups) {
        return 0;
    }

    let mut split = springs.splitn(2, |s| matches!(s, Spring::Unknown));
    if let (Some(left), Some(right)) = (split.next(), split.next()) {
        let operational: Vec<Spring> = left
            .into_iter()
            .copied()
            .chain(std::iter::once(Spring::Operational))
            .chain(right.into_iter().copied())
            .collect();
        let damaged: Vec<Spring> = left
            .into_iter()
            .copied()
            .chain(std::iter::once(Spring::Damaged))
            .chain(right.into_iter().copied())
            .collect();
        valid_arrangements(&operational, groups) + valid_arrangements(&damaged, groups)
    } else {
        // No ? left
        if is_valid_arrangement(springs, groups) { 1 } else { 0 }
    }
}

/// Returns true if the arrangement is valid (and there are no ? occurrences).
fn is_valid_arrangement(springs: &[Spring], groups: &[ContiguousGroup]) -> bool {
    let s: String = springs.iter().map(ToString::to_string).collect();
    let mut s: &str = s.as_str();

    s = match many0(tag("."))(s) {
        Ok((s, _)) => s,
        Err::<_, nom::Err<()>>(_) => return false,
    };
    let mut groups = groups.iter().peekable();
    while let Some(group) = groups.next() {
        s = match count(tag("#"), *group as usize)(s) {
            Ok((s, _)) => s,
            Err::<_, nom::Err<()>>(_) => return false,
        };

        if groups.peek().is_some() {
            s = match many1(tag("."))(s) {
                Ok((s, _)) => s,
                Err::<_, nom::Err<()>>(_) => return false,
            };
        }
    }
    s = match many0(tag("."))(s) {
        Ok((s, _)) => s,
        Err::<_, nom::Err<()>>(_) => return false,
    };

    s.is_empty()
}

/// Returns true if it can be determined that this can never result in a valid arrangement, based on the prefix of the springs string up until the first occurrence of a '?'.
fn can_be_ruled_out(springs: &[Spring], groups: &[ContiguousGroup]) -> bool {
    let s: String = springs.iter().map(ToString::to_string).collect();
    let mut s: &str = s.as_str();
    s = s.split('?').next().unwrap();

    s = match many0(nom::bytes::streaming::tag("."))(s) {
        Ok((s, _)) => s,
        Err::<_, nom::Err<()>>(err) => match err {
            nom::Err::Incomplete(_) => return false,
            _ => unreachable!(),
        },
    };
    let mut groups = groups.iter().peekable();
    while let Some(group) = groups.next() {
        s = match count(nom::bytes::streaming::tag("#"), *group as usize)(s) {
            Ok((s, _)) => s,
            Err::<_, nom::Err<()>>(err) => match err {
                nom::Err::Incomplete(_) => return false,
                _ => return true,
            },
        };

        if groups.peek().is_some() {
            s = match many1(nom::bytes::streaming::tag("."))(s) {
                Ok((s, _)) => s,
                Err::<_, nom::Err<()>>(err) => match err {
                    nom::Err::Incomplete(_) => return false,
                    _ => return true,
                },
            };
        }
    }
    false
}

mod parser {
    use super::*;
    use type_toppings::ResultExt as _;

    pub(super) fn parse(s: &str) -> Vec<Row> {
        s.lines().map(|line| all_consuming(parse_line)(line).expect_or_report(line).1).collect()
    }

    fn parse_line(s: &str) -> IResult<&str, Row> {
        let (s, springs) = many1(alt((
            map(tag("."), |_| Spring::Operational),
            map(tag("#"), |_| Spring::Damaged),
            map(tag("?"), |_| Spring::Unknown),
        )))(s)?;

        let (s, _) = tag(" ")(s)?;

        let (s, groups) = separated_list1(tag(","), u64)(s)?;

        let row = Row { springs, groups };
        Ok((s, row))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE).to_string(), "21");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE).to_string(), "");
}

#[test]
fn test_valid_arrangements() {
    let Row { springs, groups } = &parser::parse("#.#.### 1,1,3")[0];
    assert!(is_valid_arrangement(&springs, &groups));

    let Row { springs, groups } = &parser::parse("##..### 1,1,3")[0];
    assert!(!is_valid_arrangement(&springs, &groups));
}

#[test]
fn test_can_be_ruled_out() {
    let Row { springs, groups } = &parser::parse("#.#.### 1,1,3")[0];
    assert!(!can_be_ruled_out(&springs, &groups));

    let Row { springs, groups } = &parser::parse("##..### 1,1,3")[0];
    assert!(can_be_ruled_out(&springs, &groups));

    let Row { springs, groups } = &parser::parse("##..??? 1,1,3")[0];
    assert!(can_be_ruled_out(&springs, &groups));

    let Row { springs, groups } = &parser::parse("#??.### 1,1,3")[0];
    assert!(!can_be_ruled_out(&springs, &groups));
}
