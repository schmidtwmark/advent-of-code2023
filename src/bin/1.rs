use aoc::Solver;
use itertools::Itertools;

fn find_starting_digit(string: &str) -> &str {
    for i in 0..string.len() {
        let substring = &string[i..];
        if substring.chars().nth(0).unwrap().is_ascii_digit() {
            return &substring[0..1];
        } else if substring.starts_with("one") {
            return "1";
        } else if substring.starts_with("two") {
            return "2";
        } else if substring.starts_with("three") {
            return "3";
        } else if substring.starts_with("four") {
            return "4";
        } else if substring.starts_with("five") {
            return "5";
        } else if substring.starts_with("six") {
            return "6";
        } else if substring.starts_with("seven") {
            return "7";
        } else if substring.starts_with("eight") {
            return "8";
        } else if substring.starts_with("nine") {
            return "9";
        }
    }

    return "ERROR";
}

fn find_ending_digit(string: &str) -> &str {
    for i in 0..string.len() {
        let substring = &string[string.len() - 1 - i..];
        if substring.chars().nth(0).unwrap().is_ascii_digit() {
            return &substring[0..1];
        } else if substring.starts_with("one") {
            return "1";
        } else if substring.starts_with("two") {
            return "2";
        } else if substring.starts_with("three") {
            return "3";
        } else if substring.starts_with("four") {
            return "4";
        } else if substring.starts_with("five") {
            return "5";
        } else if substring.starts_with("six") {
            return "6";
        } else if substring.starts_with("seven") {
            return "7";
        } else if substring.starts_with("eight") {
            return "8";
        } else if substring.starts_with("nine") {
            return "9";
        }
    }

    return "ERROR";
}
struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        lines
            .iter()
            .map(|s| {
                let digits = s.chars().filter(|c| c.is_ascii_digit()).collect_vec();
                let num = digits.first().unwrap().to_string() + &digits.last().unwrap().to_string();
                num.parse::<usize>().unwrap()
            })
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        lines
            .iter()
            .map(|s| {
                let start_digit = find_starting_digit(s);
                let end_digit = find_ending_digit(s);
                let num = start_digit.to_string() + end_digit;
                println!("Num for string: {s} is {num}");
                num.parse::<usize>().unwrap()
            })
            .sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/1.txt");
    let sample_2 = include_str!("../../samples/1-2.txt");
    let input = include_str!("../../inputs/1.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 142),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample_2, 281),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
