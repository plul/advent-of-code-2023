//! https://adventofcode.com/2023/day/15

use type_toppings::ResultExt as _;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let steps = split_input_to_steps(input);
    steps.map(|step| step.chars().map(|c| c as usize).fold(0, hash)).sum::<usize>()
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let steps = split_input_to_steps(input).map(parse_step);
    let mut boxes: [Vec<(&str, u8)>; 256] = std::array::from_fn(|_| Vec::new());

    for step in steps {
        let mut idx = 0;
        for c in step.label().chars() {
            idx = hash(idx, c as usize);
        }
        let b = &mut boxes[idx];

        match step {
            Step::Remove(label) => {
                b.retain(|&(l, _)| label != l);
            }
            Step::Upsert(label, lens) => {
                if let Some((_, l)) = b.iter_mut().find(|(l, _)| label == *l) {
                    *l = lens;
                } else {
                    b.push((label, lens));
                }
            }
        }

        // // Debugging:
        // println!("After \"{step}\":");
        // for (i, b) in boxes.iter().enumerate().filter(|(_, b)| !b.is_empty()) {
        //     println!("Box {i}: {}", b.iter().map(|(k, v)| format!("[{k} {v}]")).collect::<Vec<_>>().join(" "));
        // }
        // println!("");
    }

    focusing_power(&boxes)
}

fn focusing_power(boxes: &[Vec<(&str, u8)>; 256]) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(box_idx, b)| {
            b.iter()
                .enumerate()
                .map(|(lens_idx, (_label, lens_focal_length))| (1 + box_idx) * (1 + lens_idx) * usize::from(*lens_focal_length))
                .sum::<usize>()
        })
        .sum()
}

fn hash(mut acc: usize, ascii_val: usize) -> usize {
    acc += ascii_val;
    acc *= 17;
    acc %= 256;
    acc
}

fn split_input_to_steps(input: &str) -> impl Iterator<Item = &str> {
    debug_assert!(input.is_ascii());
    input.lines().flat_map(|line| line.split(','))
}

enum Step<'a> {
    Remove(Label<'a>),
    Upsert(Label<'a>, Lens),
}
impl std::fmt::Display for Step<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::Remove(label) => write!(f, "{label}-"),
            Step::Upsert(label, lens) => write!(f, "{label}={lens}"),
        }
    }
}
impl<'a> Step<'a> {
    fn label(&'a self) -> Label<'a> {
        match self {
            Step::Remove(label) => label,
            Step::Upsert(label, _) => label,
        }
    }
}

type Label<'a> = &'a str;
type Lens = u8;

fn parse_step(step: &str) -> Step<'_> {
    use crate::my_nom_prelude::*;

    let remove = terminated(alpha1, tag("-"));
    let upsert = separated_pair(alpha1, tag("="), u8);

    let result: IResult<&str, Step> = all_consuming(alt((
        map(remove, |label| Step::Remove(label)),
        map(upsert, |(label, lens)| Step::Upsert(label, lens)),
    )))(step);

    result.expect_or_report(step).1
}

#[cfg(test)]
static EXAMPLE: &str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE).to_string(), "1320");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE).to_string(), "145");
}
