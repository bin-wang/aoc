use std::fs;

use itertools::Itertools;

use crate::Solution;

use super::Day10;

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

fn get_completion_cost(s: &str) -> u128 {
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
        costs.sort_unstable();
        let mid = costs.len() / 2;
        costs[mid].to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::get_test_file;
    use crate::Solution;

    use super::Day10;

    #[test]
    fn test_part_1_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day10 {}.part_1(&test_file), "26397");
    }

    #[test]
    fn test_part_1_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day10 {}.part_1(&test_file), "321237");
    }

    #[test]
    fn test_part_2_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day10 {}.part_2(&test_file), "288957");
    }

    #[test]
    fn test_part_2_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day10 {}.part_2(&test_file), "2360030859");
    }
}
