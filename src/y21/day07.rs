use itertools::Itertools;
use std::fs;

use crate::y21::Day07;
use aoc::Solution;

fn read_input(input_file: &str) -> Vec<u32> {
    let contents = fs::read_to_string(input_file).unwrap();
    contents
        .trim()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn calculate_fuel_cost(a: &Vec<u32>, t: u32) -> u32 {
    a.iter()
        .map(|x| {
            if *x < t {
                (t - x) * (t - x + 1) / 2
            } else {
                (x - t) * (x - t + 1) / 2
            }
        })
        .sum()
}

impl Solution for Day07 {
    fn part_1(&self, input_file: &str) -> String {
        let mut p = read_input(input_file);
        p.sort();

        let mut sum = 0;
        loop {
            if p.len() <= 1 {
                break;
            } else {
                let left = p.remove(0);
                let right = p.pop().unwrap();
                sum += right - left;
            }
        }

        sum.to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let p = read_input(input_file);
        let mean = p.iter().sum::<u32>() as f64 / p.len() as f64;
        let lower = (mean - 1.0).ceil() as u32;
        let upper = (mean + 1.0).floor() as u32;
        let mut costs = (lower..upper)
            .map(|t| calculate_fuel_cost(&p, t))
            .collect_vec();
        costs.sort();
        costs.last().unwrap().to_string()
    }
}
