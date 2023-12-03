use std::collections::HashMap;

use super::Solution;

#[derive(Debug, PartialEq, Eq, Hash)]
enum CubeColour {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for CubeColour {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(CubeColour::Red),
            "green" => Ok(CubeColour::Green),
            "blue" => Ok(CubeColour::Blue),
            _ => Err(anyhow::anyhow!("Invalid colour {}", value)),
        }
    }
}

pub struct Day2A;

impl Solution for Day2A {
    fn run(&self, input: &str) -> Result<String, anyhow::Error> {
        let mut sum = 0;
        for l in input.lines() {
            let (game, draws) = l.split_once(':').expect("Each line should have a :");
            let (_, game_id) = game.split_once(' ').expect("Games should end with an id");
            let game_id = game_id
                .parse::<u32>()
                .expect("Games should have valid numeric id");

            let mut game_max_cubes: HashMap<CubeColour, u8> = HashMap::new();
            let draws = draws.split("; ");
            for d in draws {
                d.split(", ").for_each(|colour_draw| {
                    let (n, colour) = colour_draw.trim()
                        .split_once(' ')
                        .expect("Correct format for colour draws");
                    let n = n.parse::<u8>().expect("Counts should be valid numbers");
                    let cube_colour: CubeColour = colour.try_into().expect("Only valid colours");
                    game_max_cubes
                        .entry(cube_colour)
                        .and_modify(|v| {
                            if *v < n {
                                *v = n;
                            }
                        })
                        .or_insert(n);
                });
            }

            if game_max_cubes.get(&CubeColour::Blue).unwrap() <= &14
                && game_max_cubes.get(&CubeColour::Green).unwrap() <= &13
                && game_max_cubes.get(&CubeColour::Red).unwrap() <= &12
            {
                sum += game_id;
            }
        }

        Ok(sum.to_string())
    }
}

pub struct Day2B;

impl Solution for Day2B {
    fn run(&self, input: &str) -> Result<String, anyhow::Error> {
        let mut sum = 0;
        for l in input.lines() {
            let (game, draws) = l.split_once(':').expect("Each line should have a :");
            let (_, game_id) = game.split_once(' ').expect("Games should end with an id");
            let game_id = game_id
                .parse::<u32>()
                .expect("Games should have valid numeric id");

            let mut game_max_cubes: HashMap<CubeColour, u32> = HashMap::new();
            let draws = draws.split("; ");
            for d in draws {
                d.split(", ").for_each(|colour_draw| {
                    let (n, colour) = colour_draw.trim()
                        .split_once(' ')
                        .expect("Correct format for colour draws");
                    let n = n.parse::<u32>().expect("Counts should be valid numbers");
                    let cube_colour: CubeColour = colour.try_into().expect("Only valid colours");
                    game_max_cubes
                        .entry(cube_colour)
                        .and_modify(|v| {
                            if *v < n {
                                *v = n;
                            }
                        })
                        .or_insert(n);
                });
            }

            let max_red = game_max_cubes.get(&CubeColour::Red).unwrap();
            let max_green = game_max_cubes.get(&CubeColour::Green).unwrap();
            let max_blue = game_max_cubes.get(&CubeColour::Blue).unwrap();

            sum += max_red * max_green * max_blue;
        }

        Ok(sum.to_string())
    }
}
