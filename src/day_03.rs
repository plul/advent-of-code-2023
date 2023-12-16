//! https://adventofcode.com/2023/day/3

use std::cmp::max;
use std::cmp::min;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum = 0;

    for (row, line) in lines.iter().enumerate() {
        let mut n = 0;
        let mut is_part_number = false;

        for (col, char) in line.iter().enumerate() {
            if let Some(d) = char.to_digit(10) {
                n *= 10;
                n += d;

                // Check for adjacent symbols
                let row_lower = max(1, row) - 1;
                let row_upper = min(lines.len() - 1, row + 1);
                let col_lower = max(1, col) - 1;
                let col_upper = min(line.len() - 1, col + 1);
                for r in row_lower..=row_upper {
                    for c in col_lower..=col_upper {
                        let x = lines[r][c];
                        if x != '.' && !x.is_ascii_digit() {
                            is_part_number = true;
                        }
                    }
                }
            } else {
                if is_part_number {
                    sum += n;
                }

                n = 0;
                is_part_number = false;
            }
        }

        if is_part_number {
            sum += n;
        }
    }

    sum
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if *char == '*' {
                let row_lower = max(1, row) - 1;
                let row_upper = min(lines.len() - 1, row + 1);
                let col_lower = max(1, col) - 1;
                let col_upper = min(line.len() - 1, col + 1);
                let mut part_numbers: Vec<PartNumber> = vec![];
                for r in row_lower..=row_upper {
                    for c in col_lower..=col_upper {
                        if !lines[r][c].is_ascii_digit() {
                            continue;
                        }
                        // Expand left
                        let mut left = c;
                        while left > 0 && lines[r][left - 1].is_ascii_digit() {
                            left -= 1;
                        }
                        // Expand right
                        let mut right = c;
                        while right + 1 < lines[r].len() && lines[r][right + 1].is_ascii_digit() {
                            right += 1;
                        }
                        let mut part_number = 0;
                        for i in left..=right {
                            part_number *= 10;
                            part_number += lines[r][i].to_digit(10).unwrap();
                        }
                        let part_number = PartNumber {
                            row: r,
                            first_digit_col: left,
                            value: part_number,
                        };
                        if !part_numbers
                            .iter().any(|p| p.row == part_number.row && p.first_digit_col == part_number.first_digit_col)
                        {
                            part_numbers.push(part_number);
                        }
                    }
                }
                match part_numbers.len() {
                    0 => {}
                    1 => {}
                    2 => {
                        sum += part_numbers[0].value * part_numbers[1].value;
                    }
                    _ => panic!("Gear surrounded by more than two part numbers"),
                }
            }
        }
    }
    sum
}

struct PartNumber {
    /// zero-indexed line number
    row: usize,

    /// zero-indexed column of first digit
    first_digit_col: usize,

    value: u32,
}

#[cfg(test)]
static EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE).to_string(), "4361");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE).to_string(), "467835");
}
