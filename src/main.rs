use crate::day1::Day1;
use crate::day2::Day2;

mod day1;
mod day2;

pub trait Day {
    type T;

    fn example_data() -> Vec<Self::T>;
    fn puzzle_data() -> Vec<Self::T>;
    fn part1(data: Vec<Self::T>) -> String;
    fn part2(data: Vec<Self::T>) -> String;
}

pub trait Report {
    fn report_part_1() -> String;
    fn report_part_2() -> String;
    fn report_day() -> String;
}

pub trait Named {
    fn name() -> String;
}

impl<T> Report for T
where
    T: Day,
    T: Named,
{
    fn report_part_1() -> String {
        let string = format!(
            "Example dataset. {}\nPuzzle dataset. {}",
            T::part1(T::example_data()),
            T::part1(T::puzzle_data())
        );
        string
    }

    fn report_part_2() -> String {
        format!(
            "Example dataset. {}\nPuzzle dataset. {}",
            T::part2(T::example_data()),
            T::part2(T::puzzle_data())
        )
    }

    fn report_day() -> String {
        format!(
            "==== {} ==== \n{}\n{}\n==== {} ====\n",
            Self::name(),
            Self::report_part_1(),
            Self::report_part_2(),
            Self::name()
        )
    }
}

fn main() {
    println!("{}", Day1::report_day());
    println!("{}", Day2::report_day());
}
