//! https://adventofcode.com/2023/day/16

use crate::lib::grid;
use crate::lib::grid::Dir;
use crate::lib::grid::Pos;
use crate::lib::grid::PosDir;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut grid = grid::vec_of_vecs::Grid::parse_char_grid(input, Tile::new);
    let pos_dir = PosDir {
        pos: Pos(0, -1),
        dir: Dir::E,
    };
    propagate_light(&mut grid, pos_dir);
    grid.into_iter().filter(|(_pos, tile)| tile.is_energized()).count()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let mut grid = grid::vec_of_vecs::Grid::parse_char_grid(input, Tile::new);
    let n_rows = grid.n_rows;
    let n_cols = grid.n_cols;

    let north_edge = (0..n_cols).map(|col| PosDir {
        pos: Pos(-1, col as isize),
        dir: Dir::S,
    });
    let west_edge = (0..n_rows).map(|row| PosDir {
        pos: Pos(row as isize, -1),
        dir: Dir::E,
    });
    let south_edge = (0..n_cols).map(|col| PosDir {
        pos: Pos(n_rows as isize, col as isize),
        dir: Dir::N,
    });
    let east_edge = (0..n_rows).map(|row| PosDir {
        pos: Pos(row as isize, n_cols as isize),
        dir: Dir::W,
    });

    let edges = north_edge.chain(west_edge).chain(south_edge).chain(east_edge);

    edges
        .map(|start| {
            grid.iter_mut().for_each(|(_pos, tile)| tile.clear());
            propagate_light(&mut grid, start);
            grid.iter().filter(|(_pos, tile)| tile.is_energized()).count()
        })
        .max()
        .unwrap()
}

fn propagate_light(grid: &mut grid::vec_of_vecs::Grid<Tile>, pos_dir: PosDir) {
    // Push & Pop from a stack, resulting in a DFS.
    let mut pos_dir_stack = vec![pos_dir];

    while let Some(mut pos_dir) = pos_dir_stack.pop() {
        pos_dir = pos_dir.step_forward();
        if !grid.contains_pos(pos_dir.pos) {
            continue;
        }

        let tile = &mut grid[pos_dir.pos];
        let energized = match pos_dir.dir {
            Dir::N => &mut tile.visited_while_moving_north,
            Dir::W => &mut tile.visited_while_moving_west,
            Dir::S => &mut tile.visited_while_moving_south,
            Dir::E => &mut tile.visited_while_moving_east,
        };
        if *energized {
            continue;
        } else {
            *energized = true;
        }

        match grid[pos_dir.pos].c {
            '.' => pos_dir_stack.push(pos_dir),
            '|' => match pos_dir.dir {
                Dir::N | Dir::S => {
                    pos_dir_stack.push(pos_dir);
                }
                Dir::W | Dir::E => {
                    pos_dir_stack.push(pos_dir.turn_left());
                    pos_dir_stack.push(pos_dir.turn_right());
                }
            },
            '-' => match pos_dir.dir {
                Dir::N | Dir::S => {
                    pos_dir_stack.push(pos_dir.turn_left());
                    pos_dir_stack.push(pos_dir.turn_right());
                }
                Dir::W | Dir::E => {
                    pos_dir_stack.push(pos_dir);
                }
            },
            '/' => match pos_dir.dir {
                Dir::N | Dir::S => {
                    pos_dir_stack.push(pos_dir.turn_right());
                }
                Dir::W | Dir::E => {
                    pos_dir_stack.push(pos_dir.turn_left());
                }
            },
            '\\' => match pos_dir.dir {
                Dir::N | Dir::S => {
                    pos_dir_stack.push(pos_dir.turn_left());
                }
                Dir::W | Dir::E => {
                    pos_dir_stack.push(pos_dir.turn_right());
                }
            },
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    c: char,
    visited_while_moving_north: bool,
    visited_while_moving_west: bool,
    visited_while_moving_south: bool,
    visited_while_moving_east: bool,
}
impl Tile {
    fn new(c: char) -> Tile {
        Tile {
            c,
            visited_while_moving_north: false,
            visited_while_moving_west: false,
            visited_while_moving_south: false,
            visited_while_moving_east: false,
        }
    }
    fn clear(&mut self) {
        *self = Tile::new(self.c);
    }
    fn is_energized(&self) -> bool {
        self.visited_while_moving_north || self.visited_while_moving_west || self.visited_while_moving_south || self.visited_while_moving_east
    }
}
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.c)
    }
}

#[cfg(test)]
fn example() -> &'static str {
    &r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#[1..] // Skip the first line ending
}

#[test]
fn part_1_example() {
    assert_eq!(part_1(example()).to_string(), "46");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(example()).to_string(), "51");
}
