use std::collections::{HashSet, HashMap};

use super::Solution;

pub struct Day3A;

impl Solution for Day3A {
    fn run(&self, input: &str) -> Result<String, anyhow::Error> {
        // Get the width of each line for later.
        let line_width = input.lines().next().and_then(|l| Some(l.len())).unwrap();
        let adj_indexes: [isize; 8] = get_adj_indexes(line_width);

        let chars: Vec<char> = input.replace("\n", "").chars().collect();

        // Collect all numbers and their locations.
        let mut numbers_map: HashMap<usize, u64> = HashMap::new();
        let mut start_i: Option<usize> = None;
        let mut end_i: Option<usize> = None;
        let mut current_n: String = String::new();
        for (i, c) in chars.iter().enumerate() {
            if c.is_numeric() {
                if start_i.is_none() {
                    start_i = Some(i);
                }
                end_i = Some(i);
                current_n.push(*c);
            } else {
                if current_n.len() > 0 {
                    for range_i in start_i.unwrap()..=end_i.unwrap() {
                        numbers_map.insert(
                            range_i,
                            current_n
                                .parse::<u64>()
                                .expect("Only valid numbers in the table"),
                        );
                    }
                }
                start_i = None;
                end_i = None;
                current_n = String::new();
            }
        }

        // Loop through the entire input, storing any numbers adj to symbols.
        let mut valid_numbers: Vec<u64> = Vec::new();
        for (i, c) in chars.iter().enumerate() {
            if c.is_ascii_punctuation() && c != &'.' {
                let mut adj_numbers: HashSet<u64> = HashSet::new();
                adj_indexes.iter().for_each(|modifier| {
                    let adj_index = ((i as isize) + modifier) as usize;
                    if adj_index > 0 && adj_index < chars.len() {
                        if let Some(adj_number) = numbers_map.get(&adj_index) {
                            adj_numbers.insert(*adj_number);
                        }
                    }
                });
                adj_numbers.iter().for_each(|n| valid_numbers.push(*n));
            }
        }

        let sum: u64 = valid_numbers.iter().sum();
        Ok(sum.to_string())
    }
}

pub struct Day3B;

impl Solution for Day3B {
    fn run(&self, input: &str) -> Result<String, anyhow::Error> {
        // Get the width of each line for later.
        let line_width = input.lines().next().and_then(|l| Some(l.len())).unwrap();
        let adj_indexes: [isize; 8] = get_adj_indexes(line_width);

        let chars: Vec<char> = input.replace("\n", "").chars().collect();

        // Collect all numbers and their locations.
        let mut numbers_map: HashMap<usize, u64> = HashMap::new();
        let mut start_i: Option<usize> = None;
        let mut end_i: Option<usize> = None;
        let mut current_n: String = String::new();
        for (i, c) in chars.iter().enumerate() {
            if c.is_numeric() {
                if start_i.is_none() {
                    start_i = Some(i);
                }
                end_i = Some(i);
                current_n.push(*c);
            } else {
                if current_n.len() > 0 {
                    for range_i in start_i.unwrap()..=end_i.unwrap() {
                        numbers_map.insert(
                            range_i,
                            current_n
                                .parse::<u64>()
                                .expect("Only valid numbers in the table"),
                        );
                    }
                }
                start_i = None;
                end_i = None;
                current_n = String::new();
            }
        }

        // Loop through the entire input, finding all symbols with two numbers adjacent.
        let mut sum: u64 = 0;
        for (i, c) in chars.iter().enumerate() {
            if c.is_ascii_punctuation() && c != &'.' {
                let mut adj_numbers: HashSet<u64> = HashSet::new();
                adj_indexes.iter().for_each(|modifier| {
                    let adj_index = ((i as isize) + modifier) as usize;
                    if adj_index > 0 && adj_index < chars.len() {
                        if let Some(adj_number) = numbers_map.get(&adj_index) {
                            adj_numbers.insert(*adj_number);
                        }
                    }
                });

                if adj_numbers.len() == 2 {
                    sum += adj_numbers.into_iter().reduce(|acc, e| acc * e).unwrap();
                }
            }
        }

        Ok(sum.to_string())
    }
}

fn get_adj_indexes(line_width: usize) -> [isize; 8] {
    let line_width = line_width as isize;
    [
        -line_width - 1,
        -line_width,
        -line_width + 1,
        - 1,
        1,
        line_width - 1,
        line_width,
        line_width + 1,
    ]
}
