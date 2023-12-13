//! https://adventofcode.com/2023/day/8

use std::collections::HashMap;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let input = parser::parse(input);
    let map = node_map(input.nodes);

    let mut node = map.get("AAA").unwrap();
    let mut steps = 0;
    for ins in input.instructions.iter().cycle() {
        if node.name == "ZZZ" {
            break;
        }

        let next = match ins {
            Ins::L => &node.left,
            Ins::R => &node.right,
        };
        node = map.get(next).unwrap();
        steps += 1;
    }

    steps
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let input = parser::parse(input);
    let map = node_map(input.nodes);

    let mut cycles: Vec<Cycle> = Vec::new();
    for start_node in map.values().filter(|n| n.name.ends_with('A')) {
        let mut node = start_node;
        let mut ins_cycle = input.instructions.iter().enumerate().cycle();

        // How many steps does it take to find the first node that ends in 'Z':
        let mut steps_to_enter_cycle = 0;
        let ins_idx_at_cycle_entry = loop {
            // Advance one step
            let (ins_idx, ins) = ins_cycle.next().unwrap();
            let next = match ins {
                Ins::L => &node.left,
                Ins::R => &node.right,
            };
            node = map.get(next).unwrap();
            steps_to_enter_cycle += 1;

            if node.name.ends_with('Z') {
                break ins_idx;
            }
        };

        // How many steps before reaching the same node again (one full cycle):
        let cycle_start = node;
        let mut cycle_len = 0;
        loop {
            // Advance one step
            let (ins_idx, ins) = ins_cycle.next().unwrap();
            let next = match ins {
                Ins::L => &node.left,
                Ins::R => &node.right,
            };
            node = map.get(next).unwrap();
            cycle_len += 1;

            // Check if we're back at the start of the cycle
            if node.name == cycle_start.name && ins_idx == ins_idx_at_cycle_entry {
                break;
            }

            if node.name.ends_with('Z') && node.name != cycle_start.name {
                // we are in trouble
                todo!("There are multiple different nodes ending in Z encountered in a single cycle.");
            }
        }

        let cycle = Cycle {
            steps_to_enter_cycle,
            cycle_len,
        };

        if cycle.cycle_len % cycle.steps_to_enter_cycle != 0 {
            todo!("Broken assumption: It's no good assuming that the cycle length is some integer multiple of the steps to enter the cycle");
        }

        cycles.push(cycle);
    }

    cycles
        .iter()
        .map(|c| c.cycle_len)
        .reduce(|c1, c2| c1 * c2 / (gcd::Gcd::gcd(c1, c2)))
        .unwrap()
}

fn node_map(nodes: Vec<Node>) -> HashMap<String, Node> {
    let mut map = HashMap::new();
    for node in nodes {
        map.insert(node.name.clone(), node);
    }
    map
}

#[derive(Debug)]
struct Cycle {
    steps_to_enter_cycle: usize,
    cycle_len: usize,
}

enum Ins {
    L,
    R,
}

struct Input {
    instructions: Vec<Ins>,
    nodes: Vec<Node>,
}

struct Node {
    name: String,
    left: String,
    right: String,
}

mod parser {
    use super::*;
    use crate::my_nom_prelude::*;
    use type_toppings::ResultExt as _;

    pub(super) fn parse(s: &str) -> Input {
        let mut lines = s.lines();
        let instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                'R' => Ins::R,
                'L' => Ins::L,
                _ => panic!(),
            })
            .collect();

        // Blank line
        lines.next();

        let nodes = lines.map(|line| all_consuming(parse_node_line)(line).expect_or_report(line).1).collect();

        Input { instructions, nodes }
    }

    fn parse_node_line(s: &str) -> IResult<&str, Node> {
        let (s, name) = alphanumeric1(s)?;
        let (s, _) = tag(" = (")(s)?;
        let (s, left) = alphanumeric1(s)?;
        let (s, _) = tag(", ")(s)?;
        let (s, right) = alphanumeric1(s)?;
        let (s, _) = tag(")")(s)?;

        let node = Node {
            name: name.to_owned(),
            left: left.to_owned(),
            right: right.to_owned(),
        };

        Ok((s, node))
    }
}

#[cfg(test)]
static EXAMPLE_1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

#[cfg(test)]
static EXAMPLE_2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

#[cfg(test)]
static EXAMPLE_3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

#[test]
fn part_1_example_1() {
    assert_eq!(part_1(EXAMPLE_1).to_string(), "2");
}

#[test]
fn part_1_example_2() {
    assert_eq!(part_1(EXAMPLE_2).to_string(), "6");
}

#[test]
fn part_2_example_3() {
    assert_eq!(part_2(EXAMPLE_3).to_string(), "6");
}
