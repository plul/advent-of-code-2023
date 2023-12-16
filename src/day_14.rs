//! https://adventofcode.com/2023/day/14

use std::collections::HashMap;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    roll_north(&mut lines);
    total_load(&lines)
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut cache_opt = Some(HashMap::new());

    let mut cycle = 1;
    while cycle <= 1000000000 {
        // Roll north
        roll_north(&mut lines);
        lines = rotate_cw(&lines);

        // Roll west
        roll_north(&mut lines);
        lines = rotate_cw(&lines);

        // Roll south
        roll_north(&mut lines);
        lines = rotate_cw(&lines);

        // Roll east
        roll_north(&mut lines);
        lines = rotate_cw(&lines);

        // Check for repetition
        if let Some(cache) = cache_opt.as_mut() {
            let key = lines.iter().flat_map(|line| line.iter()).collect::<String>();
            if let Some(cached) = cache.get(&key) {
                log::info!("Repetition detected after {cycle} cycles");

                let cycle_len = cycle - cached;

                // Fast-forward
                cycle += ((1000000000 - cycle) / cycle_len) * cycle_len;

                // Drop cache
                cache_opt = None;
            } else {
                cache.insert(key, cycle);
            }
        }

        cycle += 1;
    }

    total_load(&lines)
}

/// Rotates the grid clockwise
fn rotate_cw(lines: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n_rows = lines.len();
    let n_cols = lines[0].len();
    let mut new = vec![vec!['?'; n_rows]; n_cols];
    for row in 0..n_rows {
        for (col, item) in new.iter_mut().enumerate() {
            item[n_rows - row - 1] = lines[row][col];
        }
    }
    new
}

fn roll_north(lines: &mut Vec<Vec<char>>) {
    let n_rows = lines.len();
    let n_cols = lines[0].len();
    for col in 0..n_cols {
        let mut row_idx_1 = 0;
        'outer_while: while row_idx_1 < n_rows {
            if lines[row_idx_1][col] == '.' {
                // Empty space where a round rock can slide to.
                // Find a round rock.
                let mut row_idx_2 = row_idx_1 + 1;
                while row_idx_2 < n_rows {
                    match lines[row_idx_2][col] {
                        'O' => {
                            // Move the rock
                            lines[row_idx_1][col] = 'O';
                            lines[row_idx_2][col] = '.';
                            break;
                        }
                        '#' => {
                            row_idx_1 = row_idx_2 + 1;
                            continue 'outer_while;
                        }
                        '.' => {
                            row_idx_2 += 1;
                        }
                        _ => panic!(),
                    }
                }
            }
            row_idx_1 += 1;
        }
    }
}

fn total_load(lines: &Vec<Vec<char>>) -> usize {
    let n_rows = lines.len();
    let mut total_load = 0;
    for (r_idx, row) in lines.iter().enumerate() {
        for col in row {
            if *col == 'O' {
                total_load += n_rows - r_idx;
            }
        }
    }
    total_load
}

#[cfg(test)]
static EXAMPLE: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE).to_string(), "136");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE).to_string(), "64");
}
