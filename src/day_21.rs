//! https://adventofcode.com/2023/day/21

use crate::lib::grid;
use crate::lib::grid::Dir;
use crate::lib::grid::Pos;
use std::collections::HashSet;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    solve(input, 64)
}

pub fn solve(input: &str, steps: usize) -> impl std::fmt::Display {
    let grid = parse_grid(input);
    let (start_pos, _) = grid.iter().find(|(_pos, tile)| matches!(tile, Tile::StartingPosition)).unwrap();

    let mut s1: HashSet<Pos> = HashSet::new();
    let mut s2: HashSet<Pos> = HashSet::new();
    s2.insert(start_pos);
    for _ in 0..steps {
        (s1, s2) = (s2, s1);
        s2.clear();

        // println!();
        // grid.dbg(|pos, tile| match s1.contains(&pos) {
        //     true => 'O',
        //     false => match tile {
        //         Tile::GardenPlot => '.',
        //         Tile::Rock => '#',
        //         Tile::StartingPosition => 'S',
        //     },
        // });

        for pos in s1.iter() {
            let reachable = Dir::every_direction()
                .into_iter()
                .map(|dir| pos.step(dir))
                .filter(|p| match grid.get(*p) {
                    Some(Tile::Rock) | None => false,
                    Some(Tile::StartingPosition | Tile::GardenPlot) => true,
                });
            s2.extend(reachable);
        }
    }

    s2.len()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    ""
}

enum Tile {
    GardenPlot,
    Rock,
    StartingPosition,
}

fn parse_grid(input: &str) -> grid::vec_of_vecs::Grid<Tile> {
    grid::vec_of_vecs::Grid::<Tile>::parse_char_grid(input, |c| match c {
        '.' => Tile::GardenPlot,
        '#' => Tile::Rock,
        'S' => Tile::StartingPosition,
        _ => panic!(),
    })
}

#[cfg(test)]
fn example() -> &'static str {
    &r#"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"#[1..] // Skip the first line ending
}

#[test]
fn part_1_example() {
    assert_eq!(solve(example(), 6).to_string(), "16");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(example()).to_string(), "");
}
