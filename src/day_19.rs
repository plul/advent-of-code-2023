//! https://adventofcode.com/2023/day/19

use std::collections::HashMap;
use std::ops::Range;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let input = parser::parse(input);
    let workflow_map: WorkflowMap = HashMap::from_iter(input.workflows.iter().map(|w| (w.name, w)));
    input
        .parts
        .iter()
        .filter(|part| {
            let mut destination = Destination::Workflow("in");
            'outer: loop {
                let workflow = match destination {
                    Destination::Accepted => return true,
                    Destination::Rejected => return false,
                    Destination::Workflow(name) => workflow_map[name],
                };

                for rule in workflow.rules.iter() {
                    if let Some(dest) = apply_rule(rule, part) {
                        destination = dest;
                        continue 'outer;
                    }
                }
                destination = workflow.fallback_destination;
            }
        })
        .map(|part| part.x + part.a + part.s + part.m)
        .sum::<isize>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let input = parser::parse(input);
    let workflow_map: WorkflowMap = HashMap::from_iter(input.workflows.iter().map(|w| (w.name, w)));

    let ranges = Ranges {
        x: 1..4001,
        m: 1..4001,
        s: 1..4001,
        a: 1..4001,
    };

    let workflow = workflow_map["in"];
    let rule_idx = 0;
    accepted_configurations(&workflow_map, workflow, rule_idx, ranges)
}

fn accepted_configurations(workflow_map: &WorkflowMap<'_>, workflow: &Workflow, rule_idx: usize, ranges: Ranges) -> usize {
    if ranges.x.is_empty() || ranges.m.is_empty() || ranges.s.is_empty() || ranges.a.is_empty() {
        return 0;
    }

    let Some(rule) = workflow.rules.get(rule_idx) else {
        match workflow.fallback_destination {
            Destination::Accepted => {
                return ranges.combinations();
            }
            Destination::Rejected => {
                return 0;
            }
            Destination::Workflow(w) => {
                let workflow = workflow_map[w];
                let rule_idx = 0;
                return accepted_configurations(workflow_map, workflow, rule_idx, ranges);
            }
        }
    };

    // Split range
    let r = &ranges[rule.category];
    let (r1, r2) = match rule.operator {
        '<' => (r.start..rule.value, rule.value..r.end),
        '>' => (r.start..(rule.value + 1), (rule.value + 1)..r.end),
        _ => panic!(),
    };

    let mut ranges1 = ranges.clone();
    let mut ranges2 = ranges.clone();
    *&mut ranges1[rule.category] = r1;
    *&mut ranges2[rule.category] = r2;

    let (passed, failed) = match rule.operator {
        '<' => (ranges1, ranges2),
        '>' => (ranges2, ranges1),
        _ => panic!(),
    };

    let mut total = 0;

    // Passed rule test
    match rule.destination {
        Destination::Accepted => {
            total += passed.combinations();
        }
        Destination::Rejected => {}
        Destination::Workflow(w) => {
            let workflow = workflow_map[w];
            let rule_idx = 0;
            total += accepted_configurations(workflow_map, workflow, rule_idx, passed);
        }
    }

    // Failed rule test
    total += accepted_configurations(workflow_map, workflow, rule_idx + 1, failed);

    total
}

/// Returns destination if rule applies
fn apply_rule<'a>(rule: &'a Rule, part: &'a Part) -> Option<Destination<'a>> {
    let part_val = match rule.category {
        'x' => part.x,
        'm' => part.m,
        'a' => part.a,
        's' => part.s,
        _ => panic!(),
    };
    match rule.operator {
        '<' => part_val < rule.value,
        '>' => part_val > rule.value,
        _ => panic!(),
    }
    .then_some(rule.destination)
}

type WorkflowMap<'a> = HashMap<&'a str, &'a Workflow<'a>>;

#[derive(Debug, Clone)]
struct Ranges {
    x: Range<isize>,
    m: Range<isize>,
    a: Range<isize>,
    s: Range<isize>,
}
impl Ranges {
    fn combinations(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}
impl std::ops::Index<char> for Ranges {
    type Output = Range<isize>;
    fn index(&self, index: char) -> &Self::Output {
        match index {
            'x' => &self.x,
            'm' => &self.m,
            'a' => &self.a,
            's' => &self.s,
            _ => panic!(),
        }
    }
}
impl std::ops::IndexMut<char> for Ranges {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'x' => &mut self.x,
            'm' => &mut self.m,
            'a' => &mut self.a,
            's' => &mut self.s,
            _ => panic!(),
        }
    }
}

struct Input<'a> {
    workflows: Vec<Workflow<'a>>,
    parts: Vec<Part>,
}

struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    fallback_destination: Destination<'a>,
}

#[derive(Debug)]
struct Rule<'a> {
    category: char,
    operator: char,
    value: isize,
    destination: Destination<'a>,
}

#[derive(Copy, Clone, Debug)]
enum Destination<'a> {
    Accepted,
    Rejected,
    Workflow(&'a str),
}

struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

mod parser {
    use super::*;
    use crate::my_nom_prelude::*;
    use type_toppings::ResultExt as _;

    pub(super) fn parse(s: &str) -> Input {
        all_consuming(parse_inner)(s).unwrap_or_report().1
    }

    fn parse_inner(s: &str) -> IResult<&str, Input> {
        let (s, workflows) = many1(terminated(parse_workflow, line_ending))(s)?;
        let (s, _) = line_ending(s)?;
        let (s, parts) = many1(terminated(parse_part, line_ending))(s)?;
        Ok((s, Input { workflows, parts }))
    }

    fn parse_workflow(s: &str) -> IResult<&str, Workflow> {
        let (s, name) = alpha1(s)?;
        let (s, _) = char('{')(s)?;
        let (s, rules) = separated_list1(char(','), parse_rule)(s)?;
        let (s, _) = char(',')(s)?;
        let (s, fallback_destination) = parse_destination(s)?;
        let (s, _) = char('}')(s)?;
        let workflow = Workflow {
            name,
            rules,
            fallback_destination,
        };
        Ok((s, workflow))
    }

    fn parse_part(i: &str) -> IResult<&str, Part> {
        let (i, _) = char('{')(i)?;
        let (i, x) = preceded(tag("x="), parse_isize)(i)?;
        let (i, _) = char(',')(i)?;
        let (i, m) = preceded(tag("m="), parse_isize)(i)?;
        let (i, _) = char(',')(i)?;
        let (i, a) = preceded(tag("a="), parse_isize)(i)?;
        let (i, _) = char(',')(i)?;
        let (i, s) = preceded(tag("s="), parse_isize)(i)?;
        let (i, _) = char('}')(i)?;
        let part = Part { x, m, a, s };
        Ok((i, part))
    }

    fn parse_rule(s: &str) -> IResult<&str, Rule> {
        let (s, category) = alt((char('a'), char('m'), char('x'), char('s')))(s)?;
        let (s, operator) = alt((char('<'), char('>')))(s)?;
        let (s, value) = parse_isize(s)?;
        let (s, _) = char(':')(s)?;
        let (s, destination) = parse_destination(s)?;
        let rule = Rule {
            category,
            operator,
            value,
            destination,
        };
        Ok((s, rule))
    }

    fn parse_destination(s: &str) -> IResult<&str, Destination> {
        let (s, dest) = alt((
            value(Destination::Accepted, tag("A")),
            value(Destination::Rejected, tag("R")),
            map(take_while1(|c: char| c.is_lowercase()), |n| Destination::Workflow(n)),
        ))(s)?;
        Ok((s, dest))
    }
}

#[cfg(test)]
fn example() -> &'static str {
    &r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#[1..] // Skip the first line ending
}

#[test]
fn part_1_example() {
    assert_eq!(part_1(example()).to_string(), "19114");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(example()).to_string(), "167409079868000");
}
