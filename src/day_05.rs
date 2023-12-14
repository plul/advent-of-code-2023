//! https://adventofcode.com/2023/day/5

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let input = parser::parse(input);
    input
        .seeds
        .iter()
        .map(|&seed| {
            input.maps.iter().fold(seed, |acc, map| {
                for mapped_range in &map.mapped_ranges {
                    if let Some(dst) = mapped_range.map(acc) {
                        // Contained in a mapped range, send through mapped value
                        return dst;
                    }
                }

                // Passthrough
                acc
            })
        })
        .min()
        .unwrap()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    ""
}

#[derive(Debug)]
struct Input {
    seeds: Seeds,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    mapped_ranges: Vec<MappedRange>,
}

#[derive(Debug)]
struct MappedRange {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}
impl MappedRange {
    fn map(&self, val: usize) -> Option<usize> {
        if self.source_range_start <= val && val < self.source_range_start + self.range_length {
            Some(val + self.destination_range_start - self.source_range_start)
        } else {
            None
        }
    }
}

type Seeds = Vec<usize>;

mod parser {
    use super::*;
    use crate::my_nom_prelude::*;

    pub(super) fn parse(s: &str) -> Input {
        let (s, seeds) = parse_seeds_line(s).unwrap();
        let (_, maps) = all_consuming(separated_list1(line_ending, parse_map))(s).unwrap();
        Input { seeds, maps }
    }

    fn parse_seeds_line(s: &str) -> IResult<&str, Seeds> {
        let (s, _) = tag("seeds: ")(s)?;
        let (s, seeds) = separated_list1(tag(" "), parse_usize)(s)?;
        let (s, _) = line_ending(s)?;

        // Blank line
        let (s, _) = line_ending(s)?;

        Ok((s, seeds))
    }

    fn parse_map(s: &str) -> IResult<&str, Map> {
        let (s, _) = separated_pair(alpha1, tag("-to-"), alpha1)(s)?;
        let (s, _) = tag(" map:")(s)?;
        let (s, _) = line_ending(s)?;
        let (s, mapped_ranges) = many1(parse_mapped_range)(s)?;
        Ok((s, Map { mapped_ranges }))
    }

    fn parse_mapped_range(s: &str) -> IResult<&str, MappedRange> {
        let (s, destination_range_start) = parse_usize(s)?;
        let (s, _) = tag(" ")(s)?;
        let (s, source_range_start) = parse_usize(s)?;
        let (s, _) = tag(" ")(s)?;
        let (s, range_length) = parse_usize(s)?;
        let (s, _) = line_ending(s)?;
        Ok((
            s,
            MappedRange {
                source_range_start,
                destination_range_start,
                range_length,
            },
        ))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE).to_string(), "35");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE).to_string(), "");
}
