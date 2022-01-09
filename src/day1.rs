use crate::{Day, Named};
use aoc_2021_rust::load_to_strings;

impl Named for Day1 {
    fn name() -> String {
        String::from("Day1")
    }
}

pub struct Day1 {}

impl Day for Day1 {
    type T = i32;

    fn example_data() -> Vec<i32> {
        Self::load_data("example")
    }

    fn puzzle_data() -> Vec<i32> {
        Self::load_data("puzzle")
    }

    fn part1(data: Vec<i32>) -> String {
        let part1: i32 = count_increase(&data);
        format!("Part 1 answer: {}", part1)
    }

    fn part2(data: Vec<i32>) -> String {
        let by_three: Vec<i32> = ByThree::new(&data).collect();
        let part2: i32 = count_increase(&by_three);

        format!("Part 2 answer: {}", part2)
    }
}

impl Day1 {
    fn lookup(t: &str) -> &'static str {
        if t == "example" {
            "resources/01_example.txt"
        } else {
            "resources/01_puzzle.txt"
        }
    }

    fn load_data(data: &str) -> Vec<i32> {
        let numbers: Vec<i32> = load_to_strings(Self::lookup(data), |x| {
            x.split_whitespace().map(|x| x.parse().unwrap()).collect()
        });
        numbers
    }
}

fn count_increase(coll: &Vec<i32>) -> i32 {
    coll.iter()
        .fold((-1, 0), |acc, x| {
            (if x > &acc.1 { acc.0 + 1 } else { acc.0 }, *x)
        })
        .0
}

struct ByThree {
    coll: Vec<i32>,
    idx: usize,
}

impl ByThree {
    pub fn new(coll: &Vec<i32>) -> Self {
        Self {
            coll: coll.to_vec(),
            idx: 0,
        }
    }
}

impl Iterator for ByThree {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx > (self.coll.len() - 3) {
            None
        } else {
            let next = self
                .coll
                .iter()
                .skip(self.idx)
                .take(3)
                .fold(0, |acc, x| &acc + x);
            self.idx = self.idx + 1;
            Some(next)
        }
    }
}
