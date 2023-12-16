//! https://adventofcode.com/2023/day/5

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let input = parser::parse(input);
    input
        .seeds
        .iter()
        .map(|&seed| {
            input.maps.iter().fold(seed, |acc, map| {
                for mapped_range in &map.src_to_dst_maps {
                    if mapped_range.source_range_start <= acc && acc < mapped_range.source_range_start + mapped_range.range_length {
                        // Contained in a mapped range, send through mapped value
                        return acc + mapped_range.destination_range_start - mapped_range.source_range_start;
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
    let Input { seeds, maps } = parser::parse(input);
    let seed_ranges: Vec<Range> = seeds
        .chunks(2)
        .map(|chunk| Range {
            from: chunk[0],
            range_length: chunk[1],
        })
        .collect();

    seed_ranges
        .into_iter()
        .map(|seed_range| min_location_from_range(seed_range, &maps))
        .min()
        .unwrap()
}

fn min_location_from_range(range: Range, maps: &[Map]) -> usize {
    let Some((map, maps)) = maps.split_first() else {
        return range.from;
    };

    let mut unchanged_ranges = vec![range];
    let mut mapped_ranges = vec![];

    // single range splits into multiple ranges by being mapped
    for m in &map.src_to_dst_maps {
        let mut still_unchanged = vec![];
        for r in unchanged_ranges.iter() {
            let MapRangeOutput { unchanged, mapped } = map_range(*r, *m);
            still_unchanged.extend(unchanged);
            mapped_ranges.extend(mapped);
        }
        unchanged_ranges = still_unchanged;
    }

    let ranges: Vec<Range> = unchanged_ranges
        .into_iter()
        .chain(mapped_ranges)
        .filter(|r| r.range_length > 0)
        .collect();

    ranges.into_iter().map(|range| min_location_from_range(range, maps)).min().unwrap()
}

struct MapRangeOutput {
    /// Unaffected parts of the input range
    unchanged: Vec<Range>,

    /// The part of the input range that has been mapped to a new range
    mapped: Option<Range>,
}

/// Map range
fn map_range(range: Range, m: SrcToDstMap) -> MapRangeOutput {
    if range.contains(m.source_range_start) && range.contains(m.source_range_start + m.range_length - 1) {
        // Wholly contained

        // the part before
        let r1 = Range {
            from: range.from,
            range_length: m.source_range_start - range.from,
        };

        // the mapped part
        let r2 = Range {
            from: m.destination_range_start,
            range_length: m.range_length,
        };

        // the part after
        let r3 = Range {
            from: m.source_range_start + m.range_length,
            range_length: range.range_length - r1.range_length - r2.range_length,
        };

        return MapRangeOutput {
            unchanged: vec![r1, r3],
            mapped: Some(r2),
        };
    }

    if range.contains(m.source_range_start) {
        // Mapped range extends beyond end of 'range'

        // the part before
        let r1 = Range {
            from: range.from,
            range_length: m.source_range_start - range.from,
        };

        // the mapped part
        let r2 = Range {
            from: m.destination_range_start,
            range_length: range.range_length - r1.range_length,
        };

        return MapRangeOutput {
            unchanged: vec![r1],
            mapped: Some(r2),
        };
    }

    if range.contains(m.source_range_start + m.range_length) {
        // Mapped range starts before 'range' and ends somewhere in the middle of 'range'

        // the mapped part
        let r1 = Range {
            from: range.from + m.destination_range_start - m.source_range_start,
            range_length: m.source_range_start + m.range_length - range.from,
        };

        // the part after
        let r2 = Range {
            from: m.source_range_start + m.range_length,
            range_length: range.range_length - r1.range_length,
        };

        return MapRangeOutput {
            unchanged: vec![r2],
            mapped: Some(r1),
        };
    }

    if range.from >= m.source_range_start && range.from + range.range_length <= m.source_range_start + m.range_length {
        // 'range' is entirely within the mapped range
        let r1 = Range {
            from: range.from + m.destination_range_start - m.source_range_start,
            range_length: range.range_length,
        };
        return MapRangeOutput {
            unchanged: vec![],
            mapped: Some(r1),
        };
    }

    MapRangeOutput {
        unchanged: vec![range],
        mapped: None,
    }
}

#[derive(Debug)]
struct Input {
    seeds: Seeds,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    src_to_dst_maps: Vec<SrcToDstMap>,
}

#[derive(Debug, Clone, Copy)]
struct SrcToDstMap {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

type Seeds = Vec<usize>;

#[derive(Debug, Clone, Copy)]
struct Range {
    from: usize,
    range_length: usize,
}
impl Range {
    fn contains(&self, val: usize) -> bool {
        self.from <= val && val < self.from + self.range_length
    }
}

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
        Ok((
            s,
            Map {
                src_to_dst_maps: mapped_ranges,
            },
        ))
    }

    fn parse_mapped_range(s: &str) -> IResult<&str, SrcToDstMap> {
        let (s, destination_range_start) = parse_usize(s)?;
        let (s, _) = tag(" ")(s)?;
        let (s, source_range_start) = parse_usize(s)?;
        let (s, _) = tag(" ")(s)?;
        let (s, range_length) = parse_usize(s)?;
        let (s, _) = line_ending(s)?;
        Ok((
            s,
            SrcToDstMap {
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
    assert_eq!(part_2(EXAMPLE).to_string(), "46");
}

#[test]
fn range_contains() {
    assert!(Range { from: 64, range_length: 4 }.contains(64));
    assert!(!Range { from: 64, range_length: 4 }.contains(68));
}
