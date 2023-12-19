//! https://adventofcode.com/2023/day/18

use crate::lib::grid::Dir;
use crate::lib::grid::Pos;
use crate::lib::grid::RelDir;
use std::cmp::max;
use std::cmp::min;
use std::collections::VecDeque;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let input = parser::parse_part_1(input);
    solve(input)
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let input = parser::parse_part_2(input);
    solve(input)
}

fn solve(input: Vec<InputLine>) -> impl std::fmt::Display {
    let mut points = Vec::with_capacity(input.len());
    {
        let mut pos = Pos(0, 0);
        points.push(Some(pos));
        for line in input.iter() {
            pos = pos.steps(line.steps, line.dir);
            points.push(Some(pos));
        }
        assert_eq!(pos, Pos(0, 0), "expect we'll end up back where we started");
    }

    let mut area = 0;

    let mut hist_pos = VecDeque::<(Pos, usize)>::new();
    let mut n_points = points.len();
    'outer_outer: for buff_out in [true, false] {
        'outer: loop {
            // // Debug
            // {
            //     println!("################");
            //     let mut g = crate::lib::grid::hash_map::Grid::new();
            //     for p in points.iter().flatten() {
            //         g.insert(*p, ());
            //     }
            //     g.dbg(|t| if t.is_some() { '#' } else { '·' });

            //     for p in points.iter().flatten() {
            //         println!("({},{})", p.0, p.1);
            //     }
            //     println!("area: {area}");
            // }

            if n_points <= 4 {
                assert_eq!(n_points, 4);

                points.retain(|p| p.is_some());
                assert_eq!(points.len(), 4);

                let c0 = points[0].unwrap();
                let c1 = points[1].unwrap();
                let c2 = points[2].unwrap();
                let c3 = points[3].unwrap();

                for p in points.iter().flatten() {
                    println!("({},{})", p.0, p.1);
                }
                dbg!(area);
                let a = ((c0.row() - c2.row()).abs() + 1) * ((c0.col() - c2.col()).abs() + 1);
                dbg!(a);
                area += a;

                continue 'outer_outer;
            }

            // Clean up sparse vec
            if points.len() > 2 * n_points {
                points.retain(|p| p.is_some());
            }

            // Iterate through the points
            hist_pos.clear();
            for idx in (0..points.len()).chain(0..4) {
                let maybe_pos = &points[idx];
                let &Some(pos) = maybe_pos else {
                    continue;
                };

                let mut it = hist_pos.iter().copied();
                let pos_1 = it.next();
                let pos_2 = it.next();
                let pos_3 = it.next();
                let pos_4 = it.next();
                let pos_5 = it.next();

                // Remove sequential duplicate points
                if let Some((pos_1, pos_1_idx)) = pos_1 {
                    if pos == pos_1 {
                        points[pos_1_idx] = None;
                        n_points -= 1;
                        continue 'outer;
                    }
                }

                // Look for three points that lie on a straight line, and remove the middle point
                match (pos_1, pos_2) {
                    (Some((pos_1, pos_1_idx)), Some((pos_2, _pos_2_idx))) => {
                        let is_on_straight_line =
                            (pos.row() == pos_1.row() && pos_1.row() == pos_2.row()) || (pos.col() == pos_1.col() && pos_1.col() == pos_2.col());
                        if is_on_straight_line {
                            points[pos_1_idx] = None;
                            n_points -= 1;
                            continue 'outer;
                        }
                    }
                    _ => {}
                }

                // Look for alcoves to collapse
                'alcove: {
                    match (pos_1, pos_2, pos_3, pos_4, pos_5) {
                        (
                            Some((pos_1, pos_1_idx)),
                            Some((pos_2, pos_2_idx)),
                            Some((pos_3, pos_3_idx)),
                            Some((pos_4, pos_4_idx)),
                            Some((pos_5, pos_5_idx)),
                        ) => {
                            let dist_1 = pos.manhattan_distance(pos_1);
                            let dist_2 = pos_1.manhattan_distance(pos_2);
                            let dist_3 = pos_2.manhattan_distance(pos_3);
                            let dist_4 = pos_3.manhattan_distance(pos_4);
                            let dist_5 = pos_4.manhattan_distance(pos_5);

                            let dir_1 = Dir::from_positions(pos_1, pos).unwrap();
                            let dir_2 = Dir::from_positions(pos_2, pos_1).unwrap();
                            let dir_3 = Dir::from_positions(pos_3, pos_2).unwrap();
                            let dir_4 = Dir::from_positions(pos_4, pos_3).unwrap();
                            let dir_5 = Dir::from_positions(pos_5, pos_4).unwrap();

                            let turn_1 = RelDir::from_dirs(dir_1, dir_2);
                            let turn_2 = RelDir::from_dirs(dir_2, dir_3);
                            let turn_3 = RelDir::from_dirs(dir_3, dir_4);
                            let turn_4 = RelDir::from_dirs(dir_4, dir_5);

                            assert!(matches!(turn_1, RelDir::Left | RelDir::Right));
                            assert!(matches!(turn_2, RelDir::Left | RelDir::Right));
                            assert!(matches!(turn_3, RelDir::Left | RelDir::Right));
                            assert!(matches!(turn_4, RelDir::Left | RelDir::Right));

                            if turn_2 == turn_3 {
                                match (buff_out, turn_2, turn_3) {
                                    (false, RelDir::Left, RelDir::Left) => {
                                        // curves outward
                                    }
                                    (true, RelDir::Right, RelDir::Right) => {
                                        // curves inward
                                    }
                                    _ => {
                                        break 'alcove;
                                    }
                                };

                                let dim_1 = min(dist_2, dist_4);
                                let dim_2 = pos_2.manhattan_distance(pos_3);

                                // can be e.g. LRRL.
                                let b_1 = turn_1 == turn_4 && turn_1 != turn_2;

                                // can be e.g. ?RRL.
                                let b_2 = turn_1 != turn_2 && dist_2 < dist_4;

                                // can be e.g. LRR?.
                                let b_3 = turn_4 != turn_2 && dist_2 > dist_4;

                                if b_1 || b_2 || b_3 {
                                    let pos_2 = points[pos_2_idx].as_mut().unwrap();
                                    *pos_2 = pos_2.steps(dim_1, dir_2);

                                    let pos_3 = points[pos_3_idx].as_mut().unwrap();
                                    *pos_3 = pos_3.steps(dim_1, dir_2);

                                    let a = if buff_out {
                                        -1 * (dim_1) * (dim_2 + 1) + dim_1 * 2
                                    } else {
                                        (dim_1) * (dim_2 + 1)
                                    };
                                    area += a;

                                    continue 'outer;
                                }
                            }
                        }
                        _ => {}
                    }
                }

                if hist_pos.len() > 5 {
                    hist_pos.pop_back();
                }
                hist_pos.push_front((pos, idx));
            }

            // Didn't find anything
            continue 'outer_outer;
        }
    }

    // let path_length: isize = input.iter().map(|l| l.steps).sum();
    // area + path_length
    area
}

#[derive(Debug)]
struct InputLine {
    dir: Dir,
    steps: isize,
}

mod parser {
    use super::*;
    use crate::my_nom_prelude::*;
    use type_toppings::ResultExt as _;

    pub(super) fn parse_part_1(s: &str) -> Vec<InputLine> {
        s.lines()
            .map(|line| all_consuming(parse_line_part_1)(line).expect_or_report(line).1)
            .collect()
    }

    pub(super) fn parse_part_2(s: &str) -> Vec<InputLine> {
        s.lines()
            .map(|line| all_consuming(parse_line_part_2)(line).expect_or_report(line).1)
            .collect()
    }

    fn parse_line_part_1(s: &str) -> IResult<&str, InputLine> {
        let (s, dir) = anychar(s)?;
        let dir = match dir {
            'U' => Dir::N,
            'L' => Dir::W,
            'D' => Dir::S,
            'R' => Dir::E,
            _ => panic!(),
        };
        let (s, _) = space1(s)?;
        let (s, steps) = parse_isize(s)?;
        let (s, _) = space1(s)?;
        let (s, _) = take_while1(|c: char| !c.is_whitespace())(s)?;

        let input_line = InputLine { dir, steps };
        Ok((s, input_line))
    }

    fn parse_line_part_2(s: &str) -> IResult<&str, InputLine> {
        let (s, _) = anychar(s)?;
        let (s, _) = space1(s)?;
        let (s, _) = parse_usize(s)?;
        let (s, _) = space1(s)?;

        let (s, _) = tag("(#")(s)?;
        let (s, steps) = map_res(recognize(count(satisfy(|c| c.is_digit(16)), 5)), |h| isize::from_str_radix(h, 16))(s)?;
        let (s, dir) = map_opt(anychar, |c| match c {
            '0' => Some(Dir::E),
            '1' => Some(Dir::S),
            '2' => Some(Dir::W),
            '3' => Some(Dir::N),
            _ => None,
        })(s)?;
        let (s, _) = tag(")")(s)?;

        let input_line = InputLine { dir, steps };
        Ok((s, input_line))
    }
}

#[cfg(test)]
fn example() -> &'static str {
    &r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#[1..] // Skip the first line ending
}

#[test]
fn part_1_example() {
    assert_eq!(part_1(example()).to_string(), "62");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(example()).to_string(), "952408144115");
}

// #[test]
// fn part_1_real() {
//     let input = crate::read_input(format!("day_18.txt")).unwrap();
//     assert_eq!(part_1(&input).to_string(), "40545");
// }

#[test]
fn part_2_real() {
    let input = crate::read_input(format!("day_18.txt")).unwrap();
    assert_eq!(part_2(&input).to_string(), "90111113594927");
}
