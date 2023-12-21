//! https://adventofcode.com/2023/day/20

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let Input { mut modules, input_map: _ } = parser::parse(input);
    let mut events = VecDeque::new();

    let mut total = HashMap::<Pulse, usize>::new();
    for _ in 0..1000 {
        events.push_back(Event {
            from: None,
            to: "broadcaster",
            pulse: Pulse::Low,
        });

        while let Some(event) = events.pop_front() {
            let Event { from, to, pulse } = event;
            *total.entry(pulse).or_default() += 1;

            let Some(m) = modules.get_mut(to) else {
                continue;
            };

            let out: Option<Pulse> = match &mut m.r#type {
                Type::Broadcaster => Some(pulse),
                Type::FlipFlop { state, inputs: _ } => {
                    if matches!(pulse, Pulse::Low) {
                        state.toggle();
                        let p = match state {
                            OnOff::On => Pulse::High,
                            OnOff::Off => Pulse::Low,
                        };
                        Some(p)
                    } else {
                        None
                    }
                }
                Type::Conjunction { inputs } => {
                    let from = from.unwrap();
                    let entry = inputs.get_mut(from).unwrap();
                    *entry = pulse;
                    let p = if inputs.values().all(|p| matches!(p, Pulse::High)) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    Some(p)
                }
            };
            if let Some(p) = out {
                for dst in m.destinations.iter() {
                    events.push_back(Event {
                        from: Some(&m.name),
                        to: dst,
                        pulse: p,
                    });
                }
            }
        }
    }

    total.values().product::<usize>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let input = parser::parse(input);

    // let rx_inputs = &input.input_map["rx"];
    // for i in rx_inputs {
    // let cycle = cycle(i, &input);
    // }

    ""
}

/// Find cycle
fn cycle<'a>(name: ModuleName<'a>, input: &Input<'a>) -> Cycle {
    let m = &input.modules[name];
    match &m.r#type {
        Type::Broadcaster => Cycle { offset: 0, cycle_len: 1 },
        Type::FlipFlop { inputs, state: _ } => {
            for i in inputs {
                dbg!(i, cycle(i, input));
            }
            Cycle {
                offset: 3,    // todo
                cycle_len: 3, // todo
            }
        }
        Type::Conjunction { inputs } => {
            for (i, p) in inputs {
                dbg!(i, cycle(i, input));
            }

            Cycle {
                offset: 4,    // todo
                cycle_len: 4, // todo
            }
        }
    }
}

#[derive(Debug)]
struct Cycle {
    /// Offset from start of simulation until first low pulse.
    offset: usize,

    /// Cycle length.
    cycle_len: usize,
}

#[derive(Debug)]
struct Event<'a> {
    from: Option<ModuleName<'a>>,
    to: ModuleName<'a>,
    pulse: Pulse,
}

struct Input<'a> {
    /// Modules, indexed by name.
    modules: HashMap<ModuleName<'a>, Module<'a>>,

    input_map: HashMap<ModuleName<'a>, HashSet<ModuleName<'a>>>,
}

type ModuleName<'a> = &'a str;

#[derive(Clone, Debug, PartialEq)]
struct Module<'a> {
    name: ModuleName<'a>,
    r#type: Type<'a>,
    destinations: Vec<ModuleName<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
enum Type<'a> {
    Broadcaster,
    FlipFlop { inputs: HashSet<ModuleName<'a>>, state: OnOff },
    Conjunction { inputs: HashMap<ModuleName<'a>, Pulse> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Copy, Default)]
enum OnOff {
    On,

    #[default]
    Off,
}
impl OnOff {
    fn toggle(&mut self) {
        *self = match self {
            OnOff::On => OnOff::Off,
            OnOff::Off => OnOff::On,
        }
    }
}

mod parser {
    use super::*;
    use crate::my_nom_prelude::*;
    use type_toppings::ResultExt as _;

    pub(super) fn parse(s: &str) -> Input {
        let lines = s
            .lines()
            .map(|line| all_consuming(parse_line)(line).expect_or_report(line).1)
            .collect::<Vec<Line>>();

        let mut input_map: HashMap<ModuleName, HashSet<ModuleName>> = HashMap::new();
        for line in &lines {
            for dest in line.destinations.iter() {
                input_map.entry(dest).or_default().insert(line.name);
            }
        }

        let mut modules: HashMap<ModuleName, Module> = HashMap::new();
        for line in lines {
            modules.insert(
                line.name,
                Module {
                    name: line.name,
                    r#type: match line.r#type {
                        Type::Broadcaster => super::Type::Broadcaster,
                        Type::FlipFlop => {
                            let inputs = input_map.get(line.name).unwrap().clone();
                            super::Type::FlipFlop {
                                inputs,
                                state: OnOff::default(),
                            }
                        }
                        Type::Conjunction => {
                            let inputs = HashMap::from_iter(input_map.get(&line.name).unwrap().iter().map(|&input| (input, Pulse::Low)));
                            super::Type::Conjunction { inputs }
                        }
                    },
                    destinations: line.destinations,
                },
            );
        }

        Input { modules, input_map }
    }

    struct Line<'a> {
        name: &'a str,
        r#type: Type,
        destinations: Vec<&'a str>,
    }

    #[derive(Clone)]
    enum Type {
        Broadcaster,
        FlipFlop,
        Conjunction,
    }

    fn parse_line(s: &str) -> IResult<&str, Line> {
        let (s, (r#type, name)) = alt((
            value((Type::Broadcaster, "broadcaster"), tag("broadcaster")),
            pair(value(Type::FlipFlop, char('%')), alpha1),
            pair(value(Type::Conjunction, char('&')), alpha1),
        ))(s)?;
        let (s, _) = tag(" -> ")(s)?;
        let (s, destinations) = separated_list1(tag(", "), alpha1)(s)?;
        let line = Line { name, r#type, destinations };
        Ok((s, line))
    }
}

#[cfg(test)]
fn example_1() -> &'static str {
    &r#"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#[1..] // Skip the first line ending
}

#[cfg(test)]
fn example_2() -> &'static str {
    &r#"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#[1..] // Skip the first line ending
}

#[test]
fn part_1_example_1() {
    assert_eq!(part_1(example_1()).to_string(), "32000000");
}

#[test]
fn part_1_example_2() {
    assert_eq!(part_1(example_2()).to_string(), "11687500");
}
