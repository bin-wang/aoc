use std::fs;

use crate::solution::Solution;
use crate::y21::Day01;

impl Solution for Day01 {
    fn part_1(&self, input_file: &str) -> String {
        let input = self.read_input(input_file);
        let answer: u32 = (1..input.len())
            .map(|i| if input[i] > input[i - 1] { 1 } else { 0 })
            .sum();
        answer.to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let input = self.read_input(input_file);
        let answer: u32 = (3..input.len())
            .map(|i| if input[i] > input[i - 3] { 1 } else { 0 })
            .sum();
        answer.to_string()
    }
}

impl Day01 {
    fn read_input(&self, input_file: &str) -> Vec<u32> {
        let contents = fs::read_to_string(input_file).expect("Could not read file");
        contents
            .trim()
            .lines()
            .map(|s| s.parse().unwrap())
            .collect()
    }
}
