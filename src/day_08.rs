//! https://adventofcode.com/2023/day/8

use std::collections::HashMap;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let input = parser::parse(input);

    let mut map = HashMap::new();
    for node in input.nodes {
        map.insert(node.name.clone(), node);
    }

    let mut steps = 0;
    let mut node = map.get("AAA").unwrap();
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
    ""
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
        let (s, name) = alpha1(s)?;
        let (s, _) = tag(" = (")(s)?;
        let (s, left) = alpha1(s)?;
        let (s, _) = tag(", ")(s)?;
        let (s, right) = alpha1(s)?;
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

#[test]
fn part_1_example_1() {
    assert_eq!(part_1(EXAMPLE_1).to_string(), "2");
}

#[test]
fn part_1_example_2() {
    assert_eq!(part_1(EXAMPLE_2).to_string(), "6");
}
