//! https://adventofcode.com/2023/day/20

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let Input { mut modules } = parser::parse(input);
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
                Type::FlipFlop(f) => {
                    if matches!(pulse, Pulse::Low) {
                        f.toggle();
                        let p = match f {
                            OnOff::On => Pulse::High,
                            OnOff::Off => Pulse::Low,
                        };
                        Some(p)
                    } else {
                        None
                    }
                }
                Type::Conjunction(c) => {
                    let from = from.unwrap();
                    let entry = c.get_mut(from).unwrap();
                    *entry = pulse;
                    let p = if c.values().all(|p| matches!(p, Pulse::High)) {
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
    ""
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
    FlipFlop(OnOff),
    Conjunction(HashMap<ModuleName<'a>, Pulse>),
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
        let mut modules: HashMap<ModuleName, Module> = HashMap::new();
        let mut inputs: HashMap<ModuleName, HashSet<ModuleName>> = HashMap::new();

        for line in s.lines() {
            let module = all_consuming(parse_line)(line).expect_or_report(line).1;
            for dest in module.destinations.iter() {
                inputs.entry(dest).or_default().insert(module.name);
            }
            modules.insert(module.name, module);
        }

        for (module_name, module) in modules.iter_mut() {
            if let Type::Conjunction(c) = &mut module.r#type {
                for i in inputs.get(module_name).iter().flat_map(|set| set.iter()) {
                    c.insert(i.to_owned(), Pulse::Low);
                }
            }
        }

        Input { modules }
    }

    fn parse_line(s: &str) -> IResult<&str, Module> {
        let (s, (r#type, name)) = alt((
            value((Type::Broadcaster, "broadcaster"), tag("broadcaster")),
            pair(value(Type::FlipFlop(OnOff::default()), char('%')), alpha1),
            pair(value(Type::Conjunction(HashMap::default()), char('&')), alpha1),
        ))(s)?;
        let (s, _) = tag(" -> ")(s)?;
        let (s, destinations) = separated_list1(tag(", "), alpha1)(s)?;
        let module = Module { name, r#type, destinations };
        Ok((s, module))
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
