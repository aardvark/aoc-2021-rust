use crate::{Day, Named};

#[derive(Debug)]
pub(crate) enum Move {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Debug)]
struct Position {
    horizontal: usize,
    depth: usize,
    aim: Option<usize>,
}

impl Position {
    pub fn new(depth: usize, horizontal: usize) -> Self {
        Self {
            horizontal,
            depth,
            aim: None,
        }
    }
}

fn part1(p: Position, m: &Move) -> Position {
    match m {
        Move::Forward(amount) => Position::new(p.depth, p.horizontal + amount),
        Move::Down(amount) => Position::new(p.depth + amount, p.horizontal),
        Move::Up(amount) => Position::new(
            if p.depth.ge(amount) {
                p.depth - amount
            } else {
                0
            },
            p.horizontal,
        ),
    }
}

fn part2(p: Position, m: &Move) -> Position {
    match m {
        Move::Forward(amount) => Position {
            depth: p.depth + if let Some(n) = p.aim { n * amount } else { 0 },
            horizontal: p.horizontal + amount,
            aim: p.aim,
        },
        Move::Down(amount) => Position {
            depth: p.depth,
            horizontal: p.horizontal,
            aim: if let Some(n) = p.aim {
                Some(n + amount)
            } else {
                Some(*amount)
            },
        },
        Move::Up(amount) => Position {
            depth: p.depth,
            horizontal: p.horizontal,
            aim: if let Some(n) = p.aim {
                Some(n - amount)
            } else {
                Some(*amount)
            },
        },
    }
}

fn parse(s: &str) -> Move {
    let mut x: Vec<&str> = s.split(' ').collect();
    let direction = x.remove(0);
    let amount = x.remove(0).parse::<usize>().unwrap_or_default();
    match direction {
        "forward" => Move::Forward(amount),
        "down" => Move::Down(amount),
        "up" => Move::Up(amount),
        _ => panic!("{}", direction),
    }
}

fn load(path: &str, f: fn(&str) -> Move) -> Vec<Move> {
    let result = std::fs::read_to_string(path);
    let s = result.unwrap_or_default();
    s.lines().map(f).collect()
}

pub(crate) struct Day2 {}

impl Day for Day2 {
    type T = Move;

    fn example_data() -> Vec<Self::T> {
        load("resources/02_example.txt", parse)
    }

    fn puzzle_data() -> Vec<Self::T> {
        load("resources/02_puzzle.txt", parse)
    }

    fn part1(data: Vec<Self::T>) -> String {
        let position: Position = data.iter().fold(Position::new(0, 0), part1);

        format!(
            "{:?}. Answer: {}",
            position,
            position.horizontal * position.depth
        )
    }

    fn part2(data: Vec<Self::T>) -> String {
        let p2: Position = data.iter().fold(Position::new(0, 0), part2);

        format!("{:?}. Answer: {}", p2, p2.horizontal * p2.depth)
    }
}

impl Named for Day2 {
    fn name() -> String {
        String::from("Day 2")
    }
}
