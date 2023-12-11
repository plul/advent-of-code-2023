use clap::Parser;
use colored::Colorize;
use std::path::Path;
use std::time::Instant;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
// mod day_05;
// mod day_06;
// mod day_07;
// mod day_08;
// mod day_09;
// mod day_10;
// mod day_11;
// mod day_12;
// mod day_13;
// mod day_14;
// mod day_15;
// mod day_16;
// mod day_17;
// mod day_18;
// mod day_19;
// mod day_20;
// mod day_21;

mod my_nom_prelude {
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

mod lib {}

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
        // (5, 1) => day_05::part_1(&input?).to_string(),
        // (5, 2) => day_05::part_2(&input?).to_string(),
        // (6, 1) => day_06::part_1(&input?).to_string(),
        // (6, 2) => day_06::part_2(&input?).to_string(),
        // (7, 1) => day_07::part_1(&input?).to_string(),
        // (7, 2) => day_07::part_2(&input?).to_string(),
        // (8, 1) => day_08::part_1(&input?).to_string(),
        // (8, 2) => day_08::part_2(&input?).to_string(),
        // (9, 1) => day_09::part_1(&input?).to_string(),
        // (9, 2) => day_09::part_2(&input?).to_string(),
        // (10, 1) => day_10::part_1(&input?).to_string(),
        // (10, 2) => day_10::part_2(&input?).to_string(),
        // (11, 1) => day_11::part_1(&input?).to_string(),
        // (11, 2) => day_11::part_2(&input?).to_string(),
        // (12, 1) => day_12::part_1(&input?).to_string(),
        // (12, 2) => day_12::part_2(&input?).to_string(),
        // (13, 1) => day_13::part_1(&input?).to_string(),
        // (13, 2) => day_13::part_2(&input?).to_string(),
        // (14, 1) => day_14::part_1(&input?).to_string(),
        // (14, 2) => day_14::part_2(&input?).to_string(),
        // (15, 1) => day_15::part_1(&input?).to_string(),
        // (15, 2) => day_15::part_2(&input?).to_string(),
        // (16, 1) => day_16::part_1(&input?).to_string(),
        // (16, 2) => day_16::part_2(&input?).to_string(),
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
