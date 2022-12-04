use std::{str};

const _EXAMPLE: &str = "\
2-4,6-8\n\
2-3,4-5\n\
5-7,7-9\n\
2-8,3-7\n\
6-6,4-6\n\
2-6,4-8\n\
";

const USE_EXAMPLE: bool = false;

fn contents() -> Vec<String> {
    if USE_EXAMPLE {
        _EXAMPLE.to_owned()
    } else {
        std::fs::read_to_string("inputs/4").expect("Input not found")
    }
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .split_last()
        .expect("Input is empty")
        .1
        .to_vec()
}

type Section = u64;

#[derive(Debug)]
#[derive(Clone)]
struct Interval {
    start: Section,
    end: Section,
}

impl Interval {
    fn fully_contains(&self, other: &Interval) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Interval) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

fn parse_interval(str: &str) -> Option<Interval> {
    let (start, end) = str.split_once('-')?;
    Some(Interval{start: start.parse().ok()?, end: end.parse().ok()?})
}

fn parse_line(line: &str) -> Option<(Interval, Interval)> {
    let (left, right) = line.split_once(',')?;
    Some((parse_interval(left)?, parse_interval(right)?))
}

pub fn run() {
    let input = contents()
        .into_iter()
        .map(|s| parse_line(&s).expect(&format!("Bad input {}", s)));


    println!("{:?}", input
        .clone()
        .map(|(l, r)| l.fully_contains(&r) || r.fully_contains(&l))
        .filter(|x| *x)
        .count());

    println!("{:?}", input
        .map(|(l, r)| l.overlaps(&r))
        .filter(|x| *x)
        .count());
}
