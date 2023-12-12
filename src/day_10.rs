//! https://adventofcode.com/2023/day/10

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let lines: &Vec<Vec<char>> = &input.lines().map(|line| line.chars().collect()).collect();

    // Find start
    let mut start = None;
    'outer: for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if *char == 'S' {
                start = Some(Pos { row, col });
                break 'outer;
            }
        }
    }
    let start: Pos = start.unwrap();

    // Find initial direction
    let mut initial_direction: Dir = 'init_dir: {
        match start.go(Dir::N).and_then(|pos| char_at_pos(lines, pos)) {
            Some('|' | 'F' | '7') => break 'init_dir Dir::N,
            _ => {}
        }

        match start.go(Dir::E).and_then(|pos| char_at_pos(lines, pos)) {
            Some('-' | 'J' | '7') => break 'init_dir Dir::E,
            _ => {}
        }

        match start.go(Dir::S).and_then(|pos| char_at_pos(lines, pos)) {
            Some('|' | 'J' | 'L') => break 'init_dir Dir::S,
            _ => {}
        }

        match start.go(Dir::W).and_then(|pos| char_at_pos(lines, pos)) {
            Some('-' | 'F' | 'L') => break 'init_dir Dir::W,
            _ => {}
        }

        panic!()
    };

    // Follow the pipe
    let mut steps: usize = 0;
    let mut pos: Pos = start;
    let mut dir: Dir = initial_direction;
    loop {
        steps += 1;
        pos = pos.go(dir).unwrap();
        let c = char_at_pos(lines, pos).unwrap();
        dir = match (dir, c) {
            (_, 'S') => break,
            (Dir::N, '|') => Dir::N,
            (Dir::N, '7') => Dir::W,
            (Dir::N, 'F') => Dir::E,
            (Dir::E, '-') => Dir::E,
            (Dir::E, 'J') => Dir::N,
            (Dir::E, '7') => Dir::S,
            (Dir::S, '|') => Dir::S,
            (Dir::S, 'J') => Dir::W,
            (Dir::S, 'L') => Dir::E,
            (Dir::W, '-') => Dir::W,
            (Dir::W, 'L') => Dir::N,
            (Dir::W, 'F') => Dir::S,
            _ => panic!(),
        }
    }

    steps / 2
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    ""
}

fn char_at_pos(lines: &Vec<Vec<char>>, pos: Pos) -> Option<char> {
    lines.get(pos.row).and_then(|line| line.get(pos.col)).copied()
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
}
impl Pos {
    fn go(&self, dir: Dir) -> Option<Pos> {
        match dir {
            Dir::N => {
                if self.row == 0 {
                    None
                } else {
                    Some(Pos {
                        row: self.row - 1,
                        col: self.col,
                    })
                }
            }
            Dir::E => Some(Pos {
                row: self.row,
                col: self.col + 1,
            }),
            Dir::S => Some(Pos {
                row: self.row + 1,
                col: self.col,
            }),
            Dir::W => {
                if self.col == 0 {
                    None
                } else {
                    Some(Pos {
                        row: self.row,
                        col: self.col - 1,
                    })
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[cfg(test)]
static EXAMPLE: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE).to_string(), "8");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE).to_string(), "");
}
