use crate::day::Day;

mod day1;

pub trait Solution {
    fn run(&self, input: &str) -> Result<String, anyhow::Error>;
}

pub fn get_solution(day: &Day) -> Box<dyn Solution> {
    match day {
        Day::Day1A => Box::new(day1::Day1A),
        Day::Day1B => Box::new(day1::Day1B),
    }
}
