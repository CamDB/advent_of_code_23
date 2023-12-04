use std::{collections::{HashSet, HashMap}, time::Instant};

use super::Solution;

pub struct Day4A;

impl Solution for Day4A {
    fn run(&self, input: &str) -> Result<String, anyhow::Error> {
        let mut sum: u64 = 0;
        for l in input.lines() {
            let mut points: u64 = 0;
            let (_, numbers) = l.split_once(": ").unwrap();
            let (winning, actual) = numbers.split_once(" | ").unwrap();

            let winning_numbers: HashSet<u64> = HashSet::from_iter(winning.split_whitespace().map(|n| n.parse::<u64>().unwrap()));
            actual.split_whitespace().for_each(|n| {
                let n = n.parse::<u64>().unwrap();
                if winning_numbers.contains(&n) {
                    points = if points > 0 { points * 2 } else { 1 }
                }
            });

            sum += points;
        }

        Ok(sum.to_string())
    }
}

fn process_card(copies: &mut HashMap<usize, usize>, cached_points: Option<&usize>, index: usize, card: &str) -> usize {
    let points: usize = if let Some(points) = cached_points {
        *points
    } else {
        let (_, numbers) = card.split_once(": ").unwrap();
        let (winning, actual) = numbers.split_once(" | ").unwrap();

        let winning_numbers: HashSet<usize> = HashSet::from_iter(winning.split_whitespace().map(|n| n.parse::<usize>().unwrap()));
        let points = actual.split_whitespace().fold(0, |acc, n| {
            let n = n.parse::<usize>().unwrap();
            if winning_numbers.contains(&n) {
                acc + 1
            } else {
                acc
            }
        });

        points
    };

    for i in 0..points {
        copies.entry(i+index+1).and_modify(|n| *n += 1).or_insert(1);
    }
    
    points
}

pub struct Day4B;

impl Solution for Day4B {
    fn run(&self, input: &str) -> Result<String, anyhow::Error> {
        let mut n_cards: usize = 0;
        let mut copies: HashMap<usize, usize> = HashMap::new();
        let mut card_matches: HashMap<usize, usize> = HashMap::new();
        let cards: Vec<&str> = input.lines().collect();
        for (i, c) in cards.into_iter().enumerate() {
            let cached_points = card_matches.get(&i);
            let points = process_card(&mut copies, cached_points, i, c);
            if let None = cached_points {
                card_matches.insert(i, points);
            }

            let n_copies = *copies.get(&i).unwrap_or(&0);
            if points > 0 {
                for _ in 0..n_copies {
                    process_card(&mut copies, Some(&points), i, c);
                }
            }

            n_cards += 1 + n_copies;
        }

        Ok(n_cards.to_string())
    }
}
