use std::{collections::HashSet, thread::current};

use aoc::{Grid, Solver};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridItem {
    Digit(char),
    Empty,
    Symbol(char),
}

impl Default for GridItem {
    fn default() -> Self {
        GridItem::Empty
    }
}

fn get_number_at_pos(
    grid: &Grid<GridItem>,
    pos: (usize, usize),
) -> Option<(usize, (usize, usize))> {
    let (x, y) = pos;
    if !matches!(grid.at(pos), GridItem::Digit(_)) {
        return None;
    }

    let mut start_x = x as i64;

    while start_x >= 0 && matches!(grid.at((start_x as usize, y)), GridItem::Digit(_)) {
        start_x -= 1;
    }
    // start_x + 1 is first number
    start_x += 1;

    let mut end_x = x as i64;
    while end_x < grid.width as i64 && matches!(grid.at((end_x as usize, y)), GridItem::Digit(_)) {
        end_x += 1;
    }
    // end_x - 1 is last number
    end_x -= 1;

    let mut current_number_string = "".to_string();
    for i in start_x..=end_x {
        if let GridItem::Digit(c) = grid.at((i as usize, y)) {
            current_number_string.push(*c);
        } else {
            panic!("Expected digit, got {:?}", grid.at((i as usize, y)));
        }
    }

    Some((
        current_number_string.parse::<usize>().unwrap(),
        (start_x as usize, end_x as usize),
    ))
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let grid = Grid::from_lines(lines, &|c| match c {
            '.' => GridItem::Empty,
            '0'..='9' => GridItem::Digit(c),
            _ => GridItem::Symbol(c),
        });

        let mut current_number_string = "".to_string();
        let mut valid_number = false;
        let mut sum = 0;

        for pos in grid.positions() {
            let (x, y) = pos;
            // println!("Examining pos {pos:?}, current number string {current_number_string}, valid number {valid_number}");
            if pos.0 == 0 {
                if valid_number {
                    // println!("Adding valid number {current_number_string}");
                    sum += current_number_string.parse::<usize>().unwrap();
                }
                valid_number = false;
                current_number_string = "".to_string();
            }
            let item = grid.at(pos);
            match item {
                GridItem::Digit(c) => {
                    // println!("Is digit, {c}");
                    current_number_string.push(*c);
                    if grid
                        .neighbors(pos)
                        .any(|item| matches!(item, GridItem::Symbol(_)))
                    {
                        valid_number = true;
                    }
                }
                GridItem::Empty | GridItem::Symbol(_) => {
                    // println!("Is symbol or empty");
                    if valid_number {
                        // println!("Adding valid number {current_number_string}");
                        sum += current_number_string.parse::<usize>().unwrap();
                    }
                    valid_number = false;
                    current_number_string = "".to_string();
                }
            }
        }

        sum
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let grid = Grid::from_lines(lines, &|c| match c {
            '.' => GridItem::Empty,
            '0'..='9' => GridItem::Digit(c),
            _ => GridItem::Symbol(c),
        });

        let mut sum = 0;

        for pos in grid.positions() {
            let item = grid.at(pos);
            let (x, y) = pos;

            // Check only gears
            if matches!(item, GridItem::Symbol('*')) {
                let mut found_numbers = vec![];
                // Get the items in the top row
                if y > 0 {
                    if x > 0 {
                        found_numbers.push(get_number_at_pos(&grid, (x - 1, y - 1)));
                    }
                    found_numbers.push(get_number_at_pos(&grid, (x, y - 1)));
                    if x < grid.width - 1 {
                        found_numbers.push(get_number_at_pos(&grid, (x + 1, y - 1)));
                    }
                }

                if x > 0 {
                    found_numbers.push(get_number_at_pos(&grid, (x - 1, y)));
                }
                if x < grid.width - 1 {
                    found_numbers.push(get_number_at_pos(&grid, (x + 1, y)));
                }
                if y > 0 {
                    if x > 0 {
                        found_numbers.push(get_number_at_pos(&grid, (x - 1, y + 1)));
                    }
                    found_numbers.push(get_number_at_pos(&grid, (x, y + 1)));
                    if x < grid.width - 1 {
                        found_numbers.push(get_number_at_pos(&grid, (x + 1, y + 1)));
                    }
                }
                let filtered_numbers: HashSet<(usize, (usize, usize))> =
                    found_numbers.iter().filter_map(|n| *n).collect();
                if filtered_numbers.len() == 2 {
                    println!("Gear at {x}, {y} is valid, with numbers {filtered_numbers:?}");
                    sum += filtered_numbers
                        .iter()
                        .map(|(n, (_, _))| *n)
                        .product::<usize>();
                }
            }
        }

        sum
    }
}

fn main() {
    let sample = include_str!("../../samples/3.txt");
    let input = include_str!("../../inputs/3.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 4361),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 467835),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
