use std::{collections::HashMap, str::FromStr};

use aoc::Solver;
use itertools::Itertools;

struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl FromStr for Round {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors: HashMap<&str, usize> = s
            .split(", ")
            .map(|s| {
                let (count, color) = s
                    .split_once(' ')
                    .unwrap_or_else(|| panic!("Failed to split round at space '{}'", s));
                (
                    color,
                    count
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("Failed to parse count {}", count)),
                )
            })
            .collect();

        Ok(Round {
            red: *colors.get("red").unwrap_or(&0),
            green: *colors.get("green").unwrap_or(&0),
            blue: *colors.get("blue").unwrap_or(&0),
        })
    }
}

impl FromStr for Game {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_marker, rounds) = s.split_once(": ").unwrap();

        let id = id_marker
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<usize>()
            .unwrap_or_else(|_| panic!("Failed to parse id string {}", id_marker));

        Ok(Game {
            id,
            rounds: rounds
                .split("; ")
                .map(|s| s.parse::<Round>().unwrap())
                .collect(),
        })
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
        let games = lines
            .iter()
            .map(|s| s.parse::<Game>().unwrap())
            .collect_vec();

        games
            .iter()
            .filter(|g| {
                g.rounds
                    .iter()
                    .all(|r| r.red <= max_red && r.green <= max_green && r.blue <= max_blue)
            })
            .map(|g| g.id)
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let games = lines
            .iter()
            .map(|s| s.parse::<Game>().unwrap())
            .collect_vec();

        games
            .iter()
            .map(|g| {
                let mut min_red = 0;
                let mut min_blue = 0;
                let mut min_green = 0;

                g.rounds.iter().for_each(|r| {
                    min_red = min_red.max(r.red);
                    min_blue = min_blue.max(r.blue);
                    min_green = min_green.max(r.green);
                });

                min_red * min_blue * min_green
            })
            .sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/2.txt");
    let input = include_str!("../../inputs/2.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 8),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 2286),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
