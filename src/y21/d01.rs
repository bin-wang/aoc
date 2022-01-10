use std::fs;

use crate::Solution;

use super::Day01;

impl Solution for Day01 {
    fn part_1(&self, input_file: &str) -> String {
        let input = read_input(input_file);
        let answer: u32 = (1..input.len())
            .map(|i| if input[i] > input[i - 1] { 1 } else { 0 })
            .sum();
        answer.to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let input = read_input(input_file);
        let answer: u32 = (3..input.len())
            .map(|i| if input[i] > input[i - 3] { 1 } else { 0 })
            .sum();
        answer.to_string()
    }
}

fn read_input(input_file: &str) -> Vec<u32> {
    let contents = fs::read_to_string(input_file).unwrap();
    contents
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::test_util::get_test_file;
    use crate::Solution;

    use super::Day01;

    #[test]
    fn test_part_1_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day01 {}.part_1(&test_file), "7");
    }

    #[test]
    fn test_part_1_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day01 {}.part_1(&test_file), "1616");
    }

    #[test]
    fn test_part_2_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day01 {}.part_2(&test_file), "5");
    }

    #[test]
    fn test_part_2_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day01 {}.part_2(&test_file), "1645");
    }
}
