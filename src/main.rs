use clap::Parser;
use colored::Colorize;
use std::path::Path;
use std::time::Instant;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
// mod day_06;
// mod day_07;
mod day_08;
// mod day_09;
mod day_10;
// mod day_11;
mod day_12;
// mod day_13;
mod day_14;
mod day_15;
mod day_16;
// mod day_17;
// mod day_18;
// mod day_19;
// mod day_20;
// mod day_21;
// mod day_22;
// mod day_23;
// mod day_24;
// mod day_25;

mod my_nom_prelude {
    pub use crate::lib::nom_ext::complete::parse_usize;
    pub use nom::branch::*;
    pub use nom::bytes::complete::*;
    pub use nom::character::complete::*;
    pub use nom::character::*;
    pub use nom::combinator::*;
    pub use nom::multi::*;
    pub use nom::sequence::*;
    pub use nom::AsChar;
    pub use nom::Finish;
    pub use nom::IResult;
}

mod lib {
    /// Grid helpers for rectangular inputs
    pub mod grid {
        pub struct Grid<Tile> {
            pub rows: Vec<Vec<Tile>>,
        }
        impl<Tile> std::ops::Index<Pos> for Grid<Tile> {
            type Output = Tile;

            fn index(&self, pos: Pos) -> &Self::Output {
                let (row, col) = pos;
                assert!(row >= 0);
                assert!(col >= 0);
                let (row, col) = (row as usize, col as usize);
                let row = &self.rows[row];
                &row[col]
            }
        }
        impl<Tile> std::ops::IndexMut<Pos> for Grid<Tile> {
            fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
                let (row, col) = pos;
                assert!(row >= 0);
                assert!(col >= 0);
                let (row, col) = (row as usize, col as usize);
                let row = &mut self.rows[row];
                &mut row[col]
            }
        }
        impl<Tile> Grid<Tile> {
            pub fn n_rows(&self) -> usize {
                self.rows.len()
            }

            pub fn n_cols(&self) -> usize {
                self.rows[0].len()
            }

            pub fn contains_pos(&self, pos: Pos) -> bool {
                let (row, col) = pos;
                let n_rows = self.n_rows() as isize;
                let n_cols = self.n_cols() as isize;
                (0 <= row && row < n_rows) && (0 <= col && col < n_cols)
            }

            pub fn iter(&self) -> impl Iterator<Item = (Pos, &Tile)> {
                self.rows
                    .iter()
                    .enumerate()
                    .flat_map(|(row, cols)| cols.iter().enumerate().map(move |(col, tile)| ((row as isize, col as isize), tile)))
            }

            pub fn iter_mut(&mut self) -> impl Iterator<Item = (Pos, &mut Tile)> {
                self.rows
                    .iter_mut()
                    .enumerate()
                    .flat_map(|(row, cols)| cols.iter_mut().enumerate().map(move |(col, tile)| ((row as isize, col as isize), tile)))
            }

            pub fn into_iter(self) -> impl Iterator<Item = (Pos, Tile)> {
                self.rows
                    .into_iter()
                    .enumerate()
                    .flat_map(|(row, cols)| cols.into_iter().enumerate().map(move |(col, tile)| ((row as isize, col as isize), tile)))
            }

            #[allow(dead_code)]
            pub fn dbg<F, S>(&self, fmt: F)
            where
                F: Fn(&Tile) -> S,
                S: std::fmt::Display,
            {
                for row in &self.rows {
                    for tile in row {
                        print!("{}", fmt(tile));
                    }
                    println!();
                }
            }
        }

        /// Row, Column.
        ///
        /// (2, 8) is
        /// +-----------
        /// |
        /// |       X <- here
        /// |
        pub type Pos = (isize, isize);

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Dir {
            N,
            W,
            S,
            E,
        }

        #[derive(Clone, Copy)]
        pub struct PosDir {
            pub pos: Pos,
            pub dir: Dir,
        }
        impl std::fmt::Debug for PosDir {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "at {:?}, facing {:?}", self.pos, self.dir)
            }
        }
        impl PosDir {
            pub fn step_forward(&self) -> PosDir {
                let (row, col) = self.pos;
                let pos = match self.dir {
                    Dir::N => (row - 1, col),
                    Dir::W => (row, col - 1),
                    Dir::S => (row + 1, col),
                    Dir::E => (row, col + 1),
                };
                PosDir { pos, dir: self.dir }
            }

            pub fn turn_left(&self) -> PosDir {
                let dir = match self.dir {
                    Dir::N => Dir::W,
                    Dir::W => Dir::S,
                    Dir::S => Dir::E,
                    Dir::E => Dir::N,
                };
                PosDir { pos: self.pos, dir }
            }

            pub fn turn_right(&self) -> PosDir {
                let dir = match self.dir {
                    Dir::N => Dir::E,
                    Dir::W => Dir::N,
                    Dir::S => Dir::W,
                    Dir::E => Dir::S,
                };
                PosDir { pos: self.pos, dir }
            }
        }
    }

    /// Utility parsers for nom
    pub mod nom_ext {
        pub mod complete {
            use nom::character::complete::digit1;
            use nom::combinator::map_res;
            use nom::IResult;

            pub fn parse_usize(s: &str) -> IResult<&str, usize> {
                map_res(digit1, |s: &str| s.parse::<usize>())(s)
            }
        }
    }
}

#[derive(Parser, Debug)]
struct Cli {
    day: Option<usize>,
    part: Option<usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder().parse_default_env().init();
    let cli = Cli::parse();

    if let Some(day) = cli.day {
        if let Some(part) = cli.part {
            solve(day, part)?;
        } else {
            solve(day, 1)?;
            solve(day, 2)?;
        }
    } else {
        for day in 1..=25 {
            for part in 1..=2 {
                solve(day, part)?;
            }
        }
    }

    Ok(())
}

fn solve(day: usize, part: usize) -> Result<(), std::io::Error> {
    let input = read_input(format!("day_{day:02}.txt"));

    let now = Instant::now();
    let solution = match (day, part) {
        (1, 1) => day_01::part_1(&input?).to_string(),
        (1, 2) => day_01::part_2(&input?).to_string(),
        (2, 1) => day_02::part_1(&input?).to_string(),
        (2, 2) => day_02::part_2(&input?).to_string(),
        (3, 1) => day_03::part_1(&input?).to_string(),
        (3, 2) => day_03::part_2(&input?).to_string(),
        (4, 1) => day_04::part_1(&input?).to_string(),
        (4, 2) => day_04::part_2(&input?).to_string(),
        (5, 1) => day_05::part_1(&input?).to_string(),
        (5, 2) => day_05::part_2(&input?).to_string(),
        // (6, 1) => day_06::part_1(&input?).to_string(),
        // (6, 2) => day_06::part_2(&input?).to_string(),
        // (7, 1) => day_07::part_1(&input?).to_string(),
        // (7, 2) => day_07::part_2(&input?).to_string(),
        (8, 1) => day_08::part_1(&input?).to_string(),
        (8, 2) => day_08::part_2(&input?).to_string(),
        // (9, 1) => day_09::part_1(&input?).to_string(),
        // (9, 2) => day_09::part_2(&input?).to_string(),
        (10, 1) => day_10::part_1(&input?).to_string(),
        (10, 2) => day_10::part_2(&input?).to_string(),
        // (11, 1) => day_11::part_1(&input?).to_string(),
        // (11, 2) => day_11::part_2(&input?).to_string(),
        (12, 1) => day_12::part_1(&input?).to_string(),
        (12, 2) => day_12::part_2(&input?).to_string(),
        // (13, 1) => day_13::part_1(&input?).to_string(),
        // (13, 2) => day_13::part_2(&input?).to_string(),
        (14, 1) => day_14::part_1(&input?).to_string(),
        (14, 2) => day_14::part_2(&input?).to_string(),
        (15, 1) => day_15::part_1(&input?).to_string(),
        (15, 2) => day_15::part_2(&input?).to_string(),
        (16, 1) => day_16::part_1(&input?).to_string(),
        (16, 2) => day_16::part_2(&input?).to_string(),
        // (17, 1) => day_17::part_1(&input?).to_string(),
        // (17, 2) => day_17::part_2(&input?).to_string(),
        // (18, 1) => day_18::part_1(&input?).to_string(),
        // (18, 2) => day_18::part_2(&input?).to_string(),
        // (19, 1) => day_19::part_1(&input?).to_string(),
        // (19, 2) => day_19::part_2(&input?).to_string(),
        // (20, 1) => day_20::part_1(&input?).to_string(),
        // (20, 2) => day_20::part_2(&input?).to_string(),
        // (21, 1) => day_21::part_1(&input?).to_string(),
        // (21, 2) => day_21::part_2(&input?).to_string(),
        // (22, 1) => day_21::part_1(&input?).to_string(),
        // (22, 2) => day_21::part_2(&input?).to_string(),
        // (23, 1) => day_21::part_1(&input?).to_string(),
        // (23, 2) => day_21::part_2(&input?).to_string(),
        // (24, 1) => day_21::part_1(&input?).to_string(),
        // (24, 2) => day_21::part_2(&input?).to_string(),
        // (25, 1) => day_21::part_1(&input?).to_string(),
        // (25, 2) => day_21::part_2(&input?).to_string(),
        _ => return Ok(()),
    };
    let elapsed = now.elapsed();

    let micros = elapsed.as_micros();
    let time = match micros {
        x if x < 10_000 => format!("{}μs", micros).green(),
        x if x < 10_000_000 => format!("{}ms", micros / 1000).yellow(),
        _ => format!("{}s", micros / 1000 / 1000).red(),
    };

    println!("{time:>10}    Day {day} Part {part}: {solution}");

    Ok(())
}

fn read_input(path: impl AsRef<Path>) -> Result<String, std::io::Error> {
    std::fs::read_to_string(Path::new("input").join(path))
}
