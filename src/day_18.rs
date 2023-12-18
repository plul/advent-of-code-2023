//! https://adventofcode.com/2023/day/18

use crate::lib::grid;
use crate::lib::grid::Dir;
use crate::lib::grid::Pos;
use crate::lib::grid::PosDir;

type Grid = grid::hash_map::Grid<Tile>;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let input = parser::parse_part_1(input);
    solve(input)
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let input = parser::parse_part_2(input);
    solve(input)
}

fn solve(input: Vec<InputLine>) -> impl std::fmt::Display {
    input.windows(2).for_each(|window| {
        assert_ne!(window[0].dir, window[1].dir, "expect we'll not be making 180 degree turns");
    });

    let mut grid = Grid::new();
    let mut pos = Pos(0, 0);
    grid.insert(pos, Tile::Trench);

    let mut west_most_pos_dir = None::<PosDir>;
    let mut loop_length = 0;
    for line in &input {
        loop_length += 1;
        for _ in 0..line.steps {
            pos = pos.step(line.dir);
            grid.insert(pos, Tile::Trench);

            // To find out if the loop is CW or CCW
            if let Some(west_most_pos_dir) = &mut west_most_pos_dir {
                if pos.col() <= west_most_pos_dir.pos.col() {
                    *west_most_pos_dir = PosDir { pos, dir: line.dir };
                }
            } else {
                west_most_pos_dir = Some(PosDir { pos, dir: line.dir });
            }
        }
    }

    dbg!(loop_length);

    assert_eq!(pos, Pos(0, 0), "expect we'll end up back where we started");

    // Find out if the loop is CW or CCW
    let loop_is_clockwise = match west_most_pos_dir.unwrap().dir {
        Dir::N => true,
        Dir::S => true,
        Dir::W | Dir::E => unreachable!(),
    };

    // Traverse the loop again. This time, flood fill on the inside of the loop
    for line in &input {
        for _ in 0..line.steps {
            let dir = line.dir;
            let moving_from = pos;
            let moving_to = moving_from.step(dir);

            // Flood
            let inside: Dir = if loop_is_clockwise { dir.turn_right() } else { dir.turn_left() };
            for flood_point in [
                // next to the tile we are moving from:
                moving_from.step(inside),
                // next to the tile we are moving to:
                moving_to.step(inside),
            ]
            .into_iter()
            {
                flood(&mut grid, flood_point);
            }

            pos = moving_to;
        }
    }

    // grid.dbg(|tile| match tile {
    //     Some(Tile::Trench) => '#',
    //     None => '.',
    // });

    grid.as_ref().values().filter(|tile| matches!(tile, Tile::Trench)).count()
}

fn flood(grid: &mut Grid, pos: Pos) {
    let mut queue = vec![pos];
    while let Some(pos) = queue.pop() {
        if grid.contains_pos(&pos) {
            continue;
        }
        grid.insert(pos, Tile::Trench);
        for new_pos in Dir::every_direction().into_iter().map(|dir| pos.step(dir)) {
            queue.push(new_pos);
        }
    }
}

enum Tile {
    Trench,
}

#[derive(Debug)]
struct InputLine {
    dir: Dir,
    steps: usize,
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
        let (s, steps) = parse_usize(s)?;
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
        let (s, steps) = map_res(recognize(count(satisfy(|c| c.is_digit(16)), 5)), |h| usize::from_str_radix(h, 16))(s)?;
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
