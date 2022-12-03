use std::{str, collections::HashSet, hash::Hash};

const _EXAMPLE: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp\n\
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
PmmdzqPrVvPwwTWBwg\n\
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
ttgJtRGJQctTZtZT\n\
CrZsJsPPZsGzwwsLwLmpwMDw\n\
";

const USE_EXAMPLE: bool = false;

fn contents() -> Vec<String> {
    if USE_EXAMPLE {
        _EXAMPLE.to_owned()
    } else {
        std::fs::read_to_string("inputs/3").expect("Input not found")
    }
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .split_last()
        .expect("Input is empty")
        .1
        .to_vec()
}

type Priority = u8;

type Item = char;

trait HasPriority {
    fn priority(&self) -> Option<Priority>;
}

impl HasPriority for Item {
    fn priority(&self) -> Option<Priority> {
        if *self >= 'a' && *self <= 'z' {
            Some((*self as u8) - 96)
        } else if *self >= 'A' && *self <= 'Z' {
            Some((*self as u8 - 64) + 26)
        } else {
            None
        }
    }
}

fn split_line(line: &str) -> (Vec<Item>, Vec<Item>) {
    let mid_point = line.len() / 2;
    (line[..mid_point].chars().collect(), line[mid_point..].chars().collect())
}

fn part1(comp1: &[Item], comp2: &[Item]) -> Option<Priority> {
    comp1.iter().collect::<HashSet<_>>()
        .intersection(&comp2.iter().collect()).collect::<Vec<_>>()
        .first()
        .and_then(|common| common.priority())
}

fn intersection<T: Clone + Eq + Hash>(sets: &[HashSet<T>]) -> Option<HashSet<T>> {
    sets.into_iter().fold(
        None,
        |acc, set| acc
            .map(|intersection| intersection.intersection(set).cloned().collect())
            .or(Some(set.clone())),
    )
}

pub fn run() {
    println!("{:?}", contents()
        .iter()
        .map(|l| {
            let (comp1, comp2) = split_line(l);
            part1(&comp1, &comp2).expect("Invalid priority")
        })
        .map(|p| p as u64)
        .sum::<u64>());

    println!("{:?}", contents()
        .iter()
        .map(|s| s.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|g| intersection(g).expect(&format!("Invalid group {:?}", g)).iter().cloned().collect::<Vec<_>>())
        .map(|bs| bs.first().expect(&format!("Empty group {:?}", bs)).clone())
        .map(|b| b.priority().expect(&format!("Invalid priority {:?}", b)) as u64)
        .sum::<u64>())
}
