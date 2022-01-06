use std::fs;

use crate::y21::Day06;
use aoc::Solution;

fn progress_one_day(prev_counter: [u128; 9]) -> [u128; 9] {
    let mut counter = [0; 9];
    counter[0] = prev_counter[1];
    counter[1] = prev_counter[2];
    counter[2] = prev_counter[3];
    counter[3] = prev_counter[4];
    counter[4] = prev_counter[5];
    counter[5] = prev_counter[6];
    counter[6] = prev_counter[7] + prev_counter[0];
    counter[7] = prev_counter[8];
    counter[8] = prev_counter[0];
    counter
}

impl Solution for Day06 {
    fn part_1(&self, input_file: &str) -> String {
        let mut counter = self.read_input(input_file);
        for _ in 0..80 {
            counter = progress_one_day(counter);
        }
        counter.iter().sum::<u128>().to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let mut counter = self.read_input(input_file);
        for _ in 0..256 {
            counter = progress_one_day(counter);
        }
        counter.iter().sum::<u128>().to_string()
    }
}

impl Day06 {
    fn read_input(&self, input_file: &str) -> [u128; 9] {
        let mut counter = [0; 9];
        let contents = fs::read_to_string(input_file).unwrap();
        contents
            .trim()
            .lines()
            .map(|s| s.parse::<usize>().unwrap())
            .for_each(|i| counter[i] += 1);
        counter
    }
}
