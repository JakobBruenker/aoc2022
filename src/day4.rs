use std::str;

use crate::det_parser::DetParser;

const _EXAMPLE: &str = "\
2-4,6-8\n\
2-3,4-5\n\
5-7,7-9\n\
2-8,3-7\n\
6-6,4-6\n\
2-6,4-8\n\
";

const USE_EXAMPLE: bool = true;

type Section = u64;

struct Interval {
    start: Section,
    end: Section,
}

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
pub fn run() {
    for str in contents() {
        println!("{}", str)
    }
}
