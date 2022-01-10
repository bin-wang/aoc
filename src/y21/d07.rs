use std::fs;

use itertools::Itertools;

use crate::Solution;

use super::Day07;

fn read_input(input_file: &str) -> Vec<u32> {
    let contents = fs::read_to_string(input_file).unwrap();
    contents
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn calculate_fuel_cost(a: &[u32], t: u32) -> u32 {
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
        p.sort_unstable();

        let mut sum = 0;
        while p.len() > 1 {
            let left = p.remove(0);
            let right = p.pop().unwrap();
            sum += right - left;
        }

        sum.to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let p = read_input(input_file);
        let mean = p.iter().sum::<u32>() as f64 / p.len() as f64;
        let lower = (mean - 1.0).floor() as u32;
        let upper = (mean + 1.0).ceil() as u32;
        let mut costs = (lower..upper)
            .map(|t| calculate_fuel_cost(&p, t))
            .collect_vec();
        costs.sort_unstable();
        costs.first().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::get_test_file;
    use crate::Solution;

    use super::Day07;

    #[test]
    fn test_part_1_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day07 {}.part_1(&test_file), "37");
    }

    #[test]
    fn test_part_1_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day07 {}.part_1(&test_file), "341534");
    }

    #[test]
    fn test_part_2_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day07 {}.part_2(&test_file), "168");
    }

    #[test]
    fn test_part_2_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day07 {}.part_2(&test_file), "93397632");
    }
}
