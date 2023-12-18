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
mod day_17;
mod day_18;
// mod day_19;
// mod day_20;
// mod day_21;
// mod day_22;
// mod day_23;
// mod day_24;
// mod day_25;

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
        (17, 1) => day_17::part_1(&input?).to_string(),
        (17, 2) => day_17::part_2(&input?).to_string(),
        (18, 1) => day_18::part_1(&input?).to_string(),
        (18, 2) => day_18::part_2(&input?).to_string(),
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
    pub use nom::IResult;
}

mod lib {
    /// Grid helpers for rectangular inputs
    pub mod grid {
        pub mod hash_map {
            //! Grid implemented with `HashMap<Pos, Tile>`.
            use super::*;
            use std::cmp::max;
            use std::cmp::min;
            use std::collections::HashMap;

            #[derive(derive_more::Index, derive_more::IndexMut, derive_more::IntoIterator)]
            pub struct Grid<Tile> {
                #[index]
                #[index_mut]
                #[into_iterator(owned, ref)]
                tiles: HashMap<Pos, Tile>,

                bounds: Bounds,
            }
            impl<Tile> AsRef<HashMap<Pos, Tile>> for Grid<Tile> {
                fn as_ref(&self) -> &HashMap<Pos, Tile> {
                    &self.tiles
                }
            }
            impl<Tile> Grid<Tile> {
                pub fn new() -> Grid<Tile> {
                    Grid {
                        tiles: HashMap::new(),
                        bounds: Bounds::default(),
                    }
                }

                pub fn insert(&mut self, pos: Pos, tile: Tile) -> Option<Tile> {
                    self.expand_bounds(pos);
                    self.tiles.insert(pos, tile)
                }

                fn expand_bounds(&mut self, pos: Pos) {
                    let Pos(row, col) = pos;
                    self.bounds.min_row = min(self.bounds.min_row, row);
                    self.bounds.max_row = max(self.bounds.max_row, row);
                    self.bounds.min_col = min(self.bounds.min_col, col);
                    self.bounds.max_col = max(self.bounds.max_col, col);
                }

                pub fn contains_pos(&self, pos: &Pos) -> bool {
                    self.tiles.contains_key(pos)
                }

                pub fn bounds(&self) -> Bounds {
                    self.bounds
                }

                #[allow(dead_code)]
                pub fn dbg<F, S>(&self, fmt: F)
                where
                    F: Fn(Option<&Tile>) -> S,
                    S: std::fmt::Display,
                {
                    let Bounds {
                        min_row,
                        max_row,
                        min_col,
                        max_col,
                    } = self.bounds;
                    for row in min_row..=max_row {
                        for col in min_col..=max_col {
                            let tile = self.tiles.get(&Pos(row, col));
                            print!("{}", fmt(tile));
                        }
                        println!();
                    }
                }
            }
        }

        pub mod vec_of_vecs {
            //! Grid implemented with `Vec<Vec<Tile>>`.
            use super::*;

            pub struct Grid<Tile> {
                pub rows: Vec<Vec<Tile>>,
                pub n_rows: usize,
                pub n_cols: usize,
            }
            impl<Tile> std::ops::Index<Pos> for Grid<Tile> {
                type Output = Tile;

                fn index(&self, pos: Pos) -> &Self::Output {
                    let Pos(row, col) = pos;
                    assert!(row >= 0);
                    assert!(col >= 0);
                    let (row, col) = (row as usize, col as usize);
                    let row = &self.rows[row];
                    &row[col]
                }
            }
            impl<Tile> std::ops::IndexMut<Pos> for Grid<Tile> {
                fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
                    let Pos(row, col) = pos;
                    assert!(row >= 0);
                    assert!(col >= 0);
                    let (row, col) = (row as usize, col as usize);
                    let row = &mut self.rows[row];
                    &mut row[col]
                }
            }
            impl<Tile> Grid<Tile> {
                pub fn parse_char_grid<F>(input: &str, parse_tile: F) -> Self
                where
                    F: Fn(char) -> Tile,
                {
                    let rows: Vec<Vec<Tile>> = input.lines().map(|line| line.chars().map(&parse_tile).collect()).collect();
                    let n_rows = rows.len();
                    let n_cols = rows[0].len();
                    for row in &rows {
                        assert_eq!(row.len(), n_cols, "At least one row is not the same length as the first row");
                    }
                    Grid { rows, n_rows, n_cols }
                }

                pub fn contains_pos(&self, pos: Pos) -> bool {
                    let Pos(row, col) = pos;
                    let n_rows = self.n_rows as isize;
                    let n_cols = self.n_cols as isize;
                    (0 <= row && row < n_rows) && (0 <= col && col < n_cols)
                }

                pub fn iter(&self) -> impl Iterator<Item = (Pos, &Tile)> {
                    self.rows
                        .iter()
                        .enumerate()
                        .flat_map(|(row, cols)| cols.iter().enumerate().map(move |(col, tile)| (Pos(row as isize, col as isize), tile)))
                }

                pub fn iter_mut(&mut self) -> impl Iterator<Item = (Pos, &mut Tile)> {
                    self.rows.iter_mut().enumerate().flat_map(|(row, cols)| {
                        cols.iter_mut()
                            .enumerate()
                            .map(move |(col, tile)| (Pos(row as isize, col as isize), tile))
                    })
                }

                pub fn into_iter(self) -> impl Iterator<Item = (Pos, Tile)> {
                    self.rows.into_iter().enumerate().flat_map(|(row, cols)| {
                        cols.into_iter()
                            .enumerate()
                            .map(move |(col, tile)| (Pos(row as isize, col as isize), tile))
                    })
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
        }

        /// Row, Column.
        ///
        /// (2, 8) is
        /// +-----------
        /// |
        /// |       X <- here
        /// |
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct Pos(pub isize, pub isize);
        impl Pos {
            pub fn row(&self) -> isize {
                self.0
            }

            pub fn col(&self) -> isize {
                self.1
            }

            pub fn step(&self, dir: Dir) -> Pos {
                let Pos(row, col) = *self;
                match dir {
                    Dir::N => Pos(row - 1, col),
                    Dir::W => Pos(row, col - 1),
                    Dir::S => Pos(row + 1, col),
                    Dir::E => Pos(row, col + 1),
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Dir {
            N,
            W,
            S,
            E,
        }
        impl Dir {
            pub fn turn_left(&self) -> Dir {
                match self {
                    Dir::N => Dir::W,
                    Dir::W => Dir::S,
                    Dir::S => Dir::E,
                    Dir::E => Dir::N,
                }
            }

            pub fn turn_right(&self) -> Dir {
                match self {
                    Dir::N => Dir::E,
                    Dir::W => Dir::N,
                    Dir::S => Dir::W,
                    Dir::E => Dir::S,
                }
            }

            pub fn every_direction() -> [Dir; 4] {
                [Dir::N, Dir::E, Dir::S, Dir::W]
            }
        }

        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
                let PosDir { mut pos, dir } = *self;
                pos = pos.step(dir);
                PosDir { pos, dir }
            }

            pub fn turn_left(&self) -> PosDir {
                PosDir {
                    pos: self.pos,
                    dir: self.dir.turn_left(),
                }
            }

            pub fn turn_right(&self) -> PosDir {
                PosDir {
                    pos: self.pos,
                    dir: self.dir.turn_right(),
                }
            }
        }

        #[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct Bounds {
            pub min_row: isize,
            pub max_row: isize,
            pub min_col: isize,
            pub max_col: isize,
        }
    }

    pub mod graph {
        pub mod directed {
            //! Directed graph.

            pub trait Graph: Sized {
                /// This would typically be a ref if the graph has an inner set of stored nodes.
                ///
                /// In a dynamic graph where the nodes are computed on-the-fly, this might be an owned type.
                type Node: Node<Self>;

                /// This would typically be a ref if the graph has an inner set of stored edges.
                ///
                /// In a dynamic graph where the edges are computed on-the-fly, this might be an owned type.
                type Edge: Edge<Self>;
            }

            pub trait Node<G: Graph> {
                /// Returns edges from this node.
                fn edges(&self, graph: &G) -> impl Iterator<Item = G::Edge>;
            }

            pub trait Edge<G: Graph> {
                /// Returns the node that the edge points to.
                fn to(&self, graph: &G) -> G::Node;
            }
        }

        pub mod shortest_path {
            //! Shortest path graph algorithms.

            use super::*;
            use std::cmp::Ordering;
            use std::cmp::Reverse;
            use std::collections::BinaryHeap;
            use std::collections::HashSet;

            pub mod cost {
                use super::*;

                /// Cost of an edge in a shortest path graph traversal.
                pub trait Cost {
                    type Graph;
                    type Cost: Clone + Ord + std::ops::Add<Output = <Self as Cost>::Cost>;

                    /// Returns the cost of an edge.
                    fn cost(&self, graph: &Self::Graph) -> Self::Cost;
                }

                /// An edge that compares equality and ordering based only on cost.
                ///
                /// This effectively means the edge is a cost-compared container for other data, suitable for ordering that data in a min-heap.
                pub(super) struct Edge<G: directed::Graph>
                where
                    G::Edge: Cost,
                {
                    /// The cost of the edge, used to compare for equality and ordering.
                    pub cost: <G::Edge as Cost>::Cost,

                    /// The node the edge points to.
                    pub to_node: G::Node,
                }

                impl<G: directed::Graph> PartialEq for Edge<G>
                where
                    G::Edge: Cost,
                {
                    fn eq(&self, other: &Self) -> bool {
                        self.cost.eq(&other.cost)
                    }
                }
                impl<G: directed::Graph> Eq for Edge<G> where G::Edge: Cost {}

                impl<G: directed::Graph> PartialOrd for Edge<G>
                where
                    G::Edge: Cost,
                {
                    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                        self.cost.partial_cmp(&other.cost)
                    }
                }

                impl<G: directed::Graph> Ord for Edge<G>
                where
                    G::Edge: Cost,
                {
                    fn cmp(&self, other: &Self) -> Ordering {
                        self.cost.cmp(&other.cost)
                    }
                }
            }

            /// Returns the cost of the shortest path (if any path is found).
            ///
            /// Uses Dijkstra's algo implemented with a min-heap.
            pub fn dijkstra_min_heap<G, F>(graph: &G, start_node: G::Node, is_end: F) -> Option<<G::Edge as cost::Cost>::Cost>
            where
                G: directed::Graph,
                G::Edge: cost::Cost<Graph = G>,
                G::Node: std::hash::Hash + Eq,
                F: Fn(&G::Node) -> bool,
            {
                use cost::Cost as _;
                use cost::Edge;
                use directed::Edge as _;
                use directed::Node as _;

                // Min-heap of edges by CUMULATIVE cost.
                // Binary heap is a max-heap by default, so wrap items in Reverse to make it a min-heap.
                let mut edges: BinaryHeap<Reverse<cost::Edge<G>>> = BinaryHeap::new();

                let i = start_node
                    .edges(graph)
                    .map(|edge| Edge {
                        cost: edge.cost(graph),
                        to_node: edge.to(graph),
                    })
                    .map(Reverse);
                edges.extend(i);

                let mut visited_nodes = HashSet::<G::Node>::new();
                visited_nodes.insert(start_node);

                while let Some(Reverse(edge)) = edges.pop() {
                    if visited_nodes.contains(&edge.to_node) {
                        continue;
                    }

                    if is_end(&edge.to_node) {
                        return Some(edge.cost);
                    }

                    edges.extend(
                        edge.to_node
                            .edges(graph)
                            .filter(|e| !visited_nodes.contains(&e.to(graph)))
                            .map(|e| {
                                let cumulative_edge_cost = e.cost(graph) + edge.cost.clone();
                                Edge {
                                    to_node: e.to(graph),
                                    cost: cumulative_edge_cost,
                                }
                            })
                            .map(Reverse),
                    );
                    visited_nodes.insert(edge.to_node);
                }

                None
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
