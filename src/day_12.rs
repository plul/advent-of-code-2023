//! https://adventofcode.com/2023/day/12

use crate::my_nom_prelude::*;
use rayon::iter::IntoParallelIterator as _;
use rayon::iter::ParallelIterator as _;
use std::collections::HashMap;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    parser::parse(input)
        .into_par_iter()
        .map(|row| valid_arrangements(&row.springs, &row.groups, &mut HashMap::new()))
        .sum::<u64>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    parser::parse(input)
        .into_par_iter()
        .map(|row| {
            let mut springs = vec![];
            let mut groups = vec![];
            for _ in 0..4 {
                springs.extend(&row.springs);
                springs.push(Spring::Unknown);
                groups.extend(&row.groups);
            }
            springs.extend(row.springs);
            groups.extend(row.groups);
            Row { springs, groups }
        })
        .map(|row| valid_arrangements(&row.springs, &row.groups, &mut HashMap::new()))
        .sum::<u64>()
}

/// Contiguous group of damaged springs
type ContiguousGroup = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    groups: Vec<ContiguousGroup>,
}

/// Returns the number of different (valid) arrangements
fn valid_arrangements<'a>(
    springs: &'a [Spring],
    groups: &'a [ContiguousGroup],
    cache: &mut HashMap<(&'a [Spring], &'a [ContiguousGroup]), u64>,
) -> u64 {
    if groups.is_empty() {
        if springs.contains(&Spring::Damaged) {
            return 0;
        } else {
            return 1;
        }
    }
    if springs.is_empty() {
        return 0;
    }

    if let Some(sum) = cache.get(&(springs, groups)) {
        return *sum;
    }

    let mut sum = 0;

    // Grab the size of the next contiguous group of damaged springs
    let g = *groups.first().unwrap();

    // 1. Check if there are enough #/? to place a group here, and the following spring is not a damaged spring (because then it would have to be included in the contiguous group)
    if springs.len() >= g
        && springs.iter().take(g).all(|s| matches!(s, Spring::Damaged | Spring::Unknown))
        && !matches!(springs.get(g), Some(Spring::Damaged))
    {
        // Pop the front group
        let groups = groups.split_first().unwrap().1;

        // Pop the a number of #/? corresponding to the size of the group
        let mut springs = springs.split_at(g).1;

        // If a spring follows, pop that too, as spacing before the next group
        if !springs.is_empty() {
            springs = springs.split_first().unwrap().1;
        }

        sum += valid_arrangements(springs, groups, cache);
    }
    // 2. If the first spring here is not a damaged one, we also have the option of not placing a group here
    if !matches!(springs.first(), Some(Spring::Damaged)) && !springs.is_empty() {
        let springs = springs.split_first().unwrap().1;
        sum += valid_arrangements(springs, groups, cache);
    }

    cache.insert((springs, groups), sum);

    sum
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
        let (s, groups) = separated_list1(tag(","), parse_usize)(s)?;
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
    assert_eq!(part_2(EXAMPLE).to_string(), "525152");
}
