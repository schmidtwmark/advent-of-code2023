use std::str::FromStr;

use aoc::Solver;
use hashbrown::HashSet;
use itertools::Itertools;

struct Card {
    winning_nums: HashSet<usize>,
    my_nums: HashSet<usize>,
}

impl Card {
    fn score(&self) -> usize {
        let winner_count = self.matching_numbers();
        if winner_count == 0 {
            0
        } else {
            2_usize.pow(winner_count as u32 - 1)
        }
    }

    fn matching_numbers(&self) -> usize {
        self.winning_nums.intersection(&self.my_nums).count()
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, remainder) = s.split_once(": ").ok_or("Could not split on ': '")?;

        let (winning, mine) = remainder
            .split_once(" | ")
            .ok_or("Could not split on ' | '")?;

        let winning = winning
            .split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        let mine = mine
            .split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        Ok(Card {
            winning_nums: winning,
            my_nums: mine,
        })
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let cards = lines
            .iter()
            .filter_map(|s| Card::from_str(s).ok())
            .collect_vec();

        cards.iter().map(|c| c.score()).sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let cards = lines
            .iter()
            .filter_map(|s| Card::from_str(s).ok())
            .collect_vec();
        let mut counts = cards.iter().map(|_| 1_usize).collect_vec();

        for (card_number, card) in cards.iter().enumerate() {
            let card_count = counts[card_number];
            let score = card.matching_numbers();
            for i in 1..=score {
                counts[card_number + i] += card_count;
            }
        }

        counts.iter().sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/4.txt");
    let input = include_str!("../../inputs/4.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 13),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 30),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
