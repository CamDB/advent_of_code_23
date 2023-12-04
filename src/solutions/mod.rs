use crate::day::Day;

mod day1;
mod day2;
mod day3;
mod day4;

pub trait Solution {
    fn run(&self, input: &str) -> Result<String, anyhow::Error>;
}

pub fn get_solution(day: &Day) -> Box<dyn Solution> {
    match day {
        Day::Day1A => Box::new(day1::Day1A),
        Day::Day1B => Box::new(day1::Day1B),
        Day::Day2A => Box::new(day2::Day2A),
        Day::Day2B => Box::new(day2::Day2B),
        Day::Day3A => Box::new(day3::Day3A),
        Day::Day3B => Box::new(day3::Day3B),
        Day::Day4A => Box::new(day4::Day4A),
        Day::Day4B => Box::new(day4::Day4B),
    }
}
