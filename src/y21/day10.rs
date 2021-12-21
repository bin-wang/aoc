use itertools::Itertools;
use std::fs;

use crate::solution::Solution;
use crate::y21::Day10;

enum CheckResult {
    IllegalChar(char),
    Completion(String),
}

fn get_matching_right_parenthesis(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!(),
    }
}

fn check_line(line: &str) -> CheckResult {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(get_matching_right_parenthesis(c)),
            ')' | ']' | '}' | '>' => {
                if let Some(e) = stack.pop() {
                    if e != c {
                        return CheckResult::IllegalChar(c);
                    }
                } else {
                    return CheckResult::IllegalChar(c);
                }
            }
            _ => panic!(),
        }
    }
    stack.reverse();
    CheckResult::Completion(stack.into_iter().collect())
}

fn get_completion_cost(s: &String) -> u128 {
    s.chars()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!(),
        })
        .fold(0, |acc, v| acc * 5 + v)
}

impl Solution for Day10 {
    fn part_1(&self, input_file: &str) -> String {
        let contents = fs::read_to_string(input_file).unwrap();
        contents
            .lines()
            .map(|l| {
                if let CheckResult::IllegalChar(c) = check_line(l) {
                    match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => panic!(),
                    }
                } else {
                    0
                }
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let contents = fs::read_to_string(input_file).unwrap();
        let mut costs = contents
            .lines()
            .map(|l| {
                if let CheckResult::Completion(s) = check_line(l) {
                    get_completion_cost(&s)
                } else {
                    0
                }
            })
            .filter(|cost| *cost != 0)
            .collect_vec();
        costs.sort();
        let mid = costs.len() / 2;
        costs[mid].to_string()
    }
}
