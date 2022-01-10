use itertools::Itertools;
use std::fs;

use crate::Solution;

use super::Day06;

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

fn read_input(input_file: &str) -> [u128; 9] {
    let mut counter = [0; 9];
    let contents = fs::read_to_string(input_file).unwrap();
    let counts = contents
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .counts();
    counts.into_iter().for_each(|(k, v)| counter[k] = v as u128);
    counter
}

impl Solution for Day06 {
    fn part_1(&self, input_file: &str) -> String {
        let mut counter = read_input(input_file);
        for _ in 0..80 {
            counter = progress_one_day(counter);
        }
        counter.iter().sum::<u128>().to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let mut counter = read_input(input_file);
        for _ in 0..256 {
            counter = progress_one_day(counter);
        }
        counter.iter().sum::<u128>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::get_test_file;
    use crate::Solution;

    use super::Day06;

    #[test]
    fn test_part_1_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day06 {}.part_1(&test_file), "5934");
    }

    #[test]
    fn test_part_1_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day06 {}.part_1(&test_file), "387413");
    }

    #[test]
    fn test_part_2_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day06 {}.part_2(&test_file), "26984457539");
    }

    #[test]
    fn test_part_2_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day06 {}.part_2(&test_file), "1738377086345");
    }
}
