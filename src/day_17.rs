//! https://adventofcode.com/2023/day/17

use crate::lib::graph;
use crate::lib::graph::shortest_path::cost::Cost;
use crate::lib::grid;
use crate::lib::grid::Dir;
use crate::lib::grid::Grid;
use crate::lib::grid::Pos;
use crate::lib::grid::PosDir;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let grid = parse_grid(input);
    let graph = Graph {
        grid,
        ultra_crucibles: false,
    };
    let start_node = Node {
        pos_dir: PosDir { pos: (0, 0), dir: Dir::E },
        sequential_straight_moves: 0,
    };
    let end_pos: Pos = (graph.grid.n_rows as isize - 1, graph.grid.n_cols as isize - 1);
    let is_end = |node: &Node| node.pos_dir.pos == end_pos;
    graph::shortest_path::dijkstra_min_heap(&graph, start_node, is_end).unwrap()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let grid = parse_grid(input);
    let graph = Graph { grid, ultra_crucibles: true };
    let start_node = Node {
        pos_dir: PosDir { pos: (0, 0), dir: Dir::E },
        sequential_straight_moves: 0,
    };
    let end_pos: Pos = (graph.grid.n_rows as isize - 1, graph.grid.n_cols as isize - 1);
    let is_end = |node: &Node| node.pos_dir.pos == end_pos && node.sequential_straight_moves >= 4;
    graph::shortest_path::dijkstra_min_heap(&graph, start_node, is_end).unwrap()
}

struct Graph {
    grid: Grid<u8>,
    ultra_crucibles: bool,
}
impl graph::directed::Graph for Graph {
    type Node = Node;
    type Edge = Edge;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    /// Position and orientation.
    pos_dir: grid::PosDir,

    /// How many times it has gone in a straight line
    sequential_straight_moves: u8,
}
impl graph::directed::Node<Graph> for Node {
    fn edges(&self, graph: &Graph) -> impl Iterator<Item = <Graph as graph::directed::Graph>::Edge> {
        let mut directions = vec![];

        // Left and Right
        if !graph.ultra_crucibles || self.sequential_straight_moves >= 4 {
            directions.push(self.pos_dir.turn_left());
            directions.push(self.pos_dir.turn_right());
        }

        // Forward
        let max_sequential_straight_moves = if graph.ultra_crucibles { 10 } else { 3 };
        if self.sequential_straight_moves < max_sequential_straight_moves {
            directions.push(self.pos_dir);
        }

        directions
            .into_iter()
            .map(|pos_dir| pos_dir.step_forward())
            .filter(|pos_dir| {
                // Out of bounds check
                graph.grid.contains_pos(pos_dir.pos)
            })
            .map(|pos_dir| {
                let sequential_straight_moves = if self.pos_dir.dir == pos_dir.dir {
                    self.sequential_straight_moves + 1
                } else {
                    1
                };
                Node {
                    pos_dir,
                    sequential_straight_moves,
                }
            })
            .map(|node| Edge { to: node })
    }
}

struct Edge {
    to: Node,
}
impl graph::directed::Edge<Graph> for Edge {
    fn to(&self, _graph: &Graph) -> <Graph as graph::directed::Graph>::Node {
        self.to
    }
}
impl Cost for Edge {
    type Graph = Graph;
    type Cost = usize;

    fn cost(&self, graph: &Graph) -> Self::Cost {
        usize::from(graph.grid[self.to.pos_dir.pos])
    }
}

fn parse_grid(input: &str) -> Grid<u8> {
    Grid::<u8>::parse_char_grid(input, |c| c.to_digit(10).unwrap() as u8)
}

#[cfg(test)]
fn example() -> &'static str {
    &r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#[1..] // Skip the first line ending
}

#[cfg(test)]
fn example_2() -> &'static str {
    &r#"
111111111111
999999999991
999999999991
999999999991
999999999991
"#[1..] // Skip the first line ending
}

#[test]
fn part_1_example() {
    assert_eq!(part_1(example()).to_string(), "102");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(example()).to_string(), "94");
}

#[test]
fn part_2_example_2() {
    assert_eq!(part_2(example_2()).to_string(), "71");
}
