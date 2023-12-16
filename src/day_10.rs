//! https://adventofcode.com/2023/day/10

use std::collections::HashSet;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let lines: &Vec<Vec<char>> = &input.lines().map(|line| line.chars().collect()).collect();
    let bounds = Bounds {
        rows: lines.len(),
        cols: lines[0].len(),
    };

    trace_pipe(lines, bounds).len() / 2
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let lines: &Vec<Vec<char>> = &input.lines().map(|line| line.chars().collect()).collect();
    let bounds = Bounds {
        rows: lines.len(),
        cols: lines[0].len(),
    };

    // Find tiles that are pipe.
    let pipe: HashSet<Pos> = HashSet::from_iter(trace_pipe(lines, bounds));

    // Fill from the edges
    let mut connected_to_outer_edge: HashSet<Pos> = HashSet::new();
    let mut edges = Vec::new();
    for row in 0..lines.len() {
        // Left edge
        let pos = Pos { row, col: 0 };
        edges.push(pos);
        // Right edge
        let pos = Pos { row, col: bounds.cols - 1 };
        edges.push(pos);
    }
    for col in 0..lines[0].len() {
        // Upper edge
        let pos = Pos { row: 0, col };
        edges.push(pos);
        // Lower edge
        let pos = Pos { row: bounds.rows - 1, col };
        edges.push(pos);
    }
    for pos in edges {
        flood(&mut connected_to_outer_edge, &pipe, pos, bounds);
    }

    // Find out which side of the pipe is the outside and which side is the inside of the loop
    let (mut pos, mut dir): (Pos, Dir) = find_start(lines, bounds);
    let loop_is_cw = loop {
        let pos_to_right_hand_side = match dir {
            Dir::N => pos.go(Dir::E, bounds),
            Dir::E => pos.go(Dir::S, bounds),
            Dir::S => pos.go(Dir::W, bounds),
            Dir::W => pos.go(Dir::N, bounds),
        };
        match pos_to_right_hand_side {
            Some(p) => {
                if connected_to_outer_edge.contains(&p) {
                    break false;
                }
            }
            None => {
                // if pos is out of bounds, then we're definitely on the outside of the loop.
                break false;
            }
        }
        let (p, Some(d)) = step(lines, bounds, pos, dir) else {
            // arrived back at start
            break true;
        };
        (pos, dir) = (p, d);
    };

    // Now traverse the loop again. This time, flood fill anything deemed to be on the inside of the loop,
    let mut inside_of_loop: HashSet<Pos> = HashSet::new();
    let (mut pos, mut dir): (Pos, Dir) = find_start(lines, bounds);
    loop {
        let inside = match (loop_is_cw, dir) {
            (true, Dir::N) => Dir::E,
            (true, Dir::E) => Dir::S,
            (true, Dir::S) => Dir::W,
            (true, Dir::W) => Dir::N,
            (false, Dir::N) => Dir::W,
            (false, Dir::E) => Dir::N,
            (false, Dir::S) => Dir::E,
            (false, Dir::W) => Dir::S,
        };
        for flood_point in [
            // next to the tile we are moving from:
            pos.go(inside, bounds),
            // next to the tile we are moving to:
            pos.go(dir, bounds).and_then(|p| p.go(inside, bounds)),
        ]
        .into_iter()
        .flatten()
        {
            flood(&mut inside_of_loop, &pipe, flood_point, bounds);
        }

        let (p, Some(d)) = step(lines, bounds, pos, dir) else {
            // arrived back at start
            break;
        };
        (pos, dir) = (p, d);
    }

    inside_of_loop.len()
}

/// Floods from `pos`.
fn flood(filled: &mut HashSet<Pos>, pipe: &HashSet<Pos>, pos: Pos, bounds: Bounds) {
    if filled.contains(&pos) || pipe.contains(&pos) {
        return;
    }
    filled.insert(pos);

    for new_pos in [Dir::N, Dir::E, Dir::S, Dir::W].into_iter().filter_map(|dir| pos.go(dir, bounds)) {
        if !filled.contains(&new_pos) && !pipe.contains(&new_pos) {
            flood(filled, pipe, new_pos, bounds);
        }
    }
}

fn find_start(lines: &[Vec<char>], bounds: Bounds) -> (Pos, Dir) {
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
    let initial_direction: Dir = 'init_dir: {
        if let Some('|' | 'F' | '7') = start.go(Dir::N, bounds).and_then(|pos| char_at_pos(lines, pos)) {
            break 'init_dir Dir::N;
        }
        if let Some('-' | 'J' | '7') = start.go(Dir::E, bounds).and_then(|pos| char_at_pos(lines, pos)) {
            break 'init_dir Dir::E;
        }
        if let Some('|' | 'J' | 'L') = start.go(Dir::S, bounds).and_then(|pos| char_at_pos(lines, pos)) {
            break 'init_dir Dir::S;
        }
        if let Some('-' | 'F' | 'L') = start.go(Dir::W, bounds).and_then(|pos| char_at_pos(lines, pos)) {
            break 'init_dir Dir::W;
        }
        panic!()
    };

    (start, initial_direction)
}

/// Returns the positions of tiles that make up the pipe loop.
fn trace_pipe(lines: &[Vec<char>], bounds: Bounds) -> Vec<Pos> {
    let mut v = vec![];

    let (mut pos, mut dir): (Pos, Dir) = find_start(lines, bounds);

    // Follow the pipe
    loop {
        v.push(pos);
        let (p, Some(d)) = step(lines, bounds, pos, dir) else {
            // arrived back at start
            break;
        };
        (pos, dir) = (p, d);
    }

    v
}

/// Takes single step along pipe and returns new position and direction.
fn step(lines: &[Vec<char>], bounds: Bounds, pos: Pos, dir: Dir) -> (Pos, Option<Dir>) {
    let pos = pos.go(dir, bounds).unwrap();
    let c = char_at_pos(lines, pos).unwrap();
    let dir = match (dir, c) {
        (_, 'S') => None,
        (Dir::N, '|') => Some(Dir::N),
        (Dir::N, '7') => Some(Dir::W),
        (Dir::N, 'F') => Some(Dir::E),
        (Dir::E, '-') => Some(Dir::E),
        (Dir::E, 'J') => Some(Dir::N),
        (Dir::E, '7') => Some(Dir::S),
        (Dir::S, '|') => Some(Dir::S),
        (Dir::S, 'J') => Some(Dir::W),
        (Dir::S, 'L') => Some(Dir::E),
        (Dir::W, '-') => Some(Dir::W),
        (Dir::W, 'L') => Some(Dir::N),
        (Dir::W, 'F') => Some(Dir::S),
        _ => panic!(),
    };

    (pos, dir)
}

/// Returns the character at a given position
fn char_at_pos(lines: &[Vec<char>], pos: Pos) -> Option<char> {
    lines.get(pos.row).and_then(|line| line.get(pos.col)).copied()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
}
impl Pos {
    fn go(&self, dir: Dir, bounds: Bounds) -> Option<Pos> {
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
            Dir::E => {
                if self.col + 1 >= bounds.cols {
                    None
                } else {
                    Some(Pos {
                        row: self.row,
                        col: self.col + 1,
                    })
                }
            }
            Dir::S => {
                if self.row + 1 >= bounds.rows {
                    None
                } else {
                    Some(Pos {
                        row: self.row + 1,
                        col: self.col,
                    })
                }
            }
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

#[derive(Copy, Clone, Debug)]
struct Bounds {
    /// Number of rows in the input
    rows: usize,

    /// Number of columns in the input
    cols: usize,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[cfg(test)]
static EXAMPLE_1: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

#[cfg(test)]
static EXAMPLE_2: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

#[cfg(test)]
static EXAMPLE_3: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE_1).to_string(), "8");
}

#[test]
fn part_2_example_1() {
    assert_eq!(part_2(EXAMPLE_1).to_string(), "1");
}

#[test]
fn part_2_example_2() {
    assert_eq!(part_2(EXAMPLE_2).to_string(), "4");
}

#[test]
fn part_2_example_3() {
    assert_eq!(part_2(EXAMPLE_3).to_string(), "8");
}
