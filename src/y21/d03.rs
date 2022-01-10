use std::fs;

use itertools::Itertools;

use crate::Solution;

use super::Day03;

enum BitCriteria {
    MostCommon,
    LeastCommon,
}

impl Solution for Day03 {
    fn part_1(&self, input_file: &str) -> String {
        let contents = fs::read_to_string(input_file).unwrap();
        let input = contents
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();
        let digit_len = input.first().unwrap().len();

        let mut gamma_binary = String::new();
        let mut epsilon_binary = String::new();
        for i in 0..digit_len {
            let counts = input.iter().map(|l| l[i]).counts();
            let (least_common_digit, most_common_digit) = counts
                .iter()
                .sorted_by_key(|it| it.1)
                .map(|it| it.0)
                .collect_tuple()
                .unwrap();
            gamma_binary.push(*most_common_digit);
            epsilon_binary.push(*least_common_digit);
        }

        let gamma_rate = u32::from_str_radix(gamma_binary.as_str(), 2).unwrap();
        let epsilon_rate = u32::from_str_radix(epsilon_binary.as_str(), 2).unwrap();
        (gamma_rate * epsilon_rate).to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let contents = fs::read_to_string(input_file).unwrap();
        let input = contents.lines().collect_vec();

        let oxygen_binary = select_matching_string(&input, BitCriteria::MostCommon);
        let oxygen_rating = u32::from_str_radix(oxygen_binary.as_str(), 2).unwrap();
        let co2_binary = select_matching_string(&input, BitCriteria::LeastCommon);
        let co2_rating = u32::from_str_radix(co2_binary.as_str(), 2).unwrap();
        (oxygen_rating * co2_rating).to_string()
    }
}

fn split_first(s: &str) -> (char, &str) {
    let head = s.chars().next().unwrap();
    let tail = &s[1..];
    (head, tail)
}

fn select_matching_string(number_binary_reprs: &[&str], criteria: BitCriteria) -> String {
    if number_binary_reprs.len() == 1 {
        number_binary_reprs.first().unwrap().to_string()
    } else {
        let tails_grouped_by_first_char = number_binary_reprs
            .iter()
            .map(|s| split_first(s))
            .into_group_map();

        let filter = match criteria {
            BitCriteria::MostCommon => {
                tails_grouped_by_first_char
                    .iter()
                    .max_by_key(|it| (it.1.len(), it.0))
                    .unwrap()
                    .0
            }
            BitCriteria::LeastCommon => {
                tails_grouped_by_first_char
                    .iter()
                    .min_by_key(|it| (it.1.len(), it.0))
                    .unwrap()
                    .0
            }
        };

        let mut ans = select_matching_string(&tails_grouped_by_first_char[filter], criteria);
        ans.insert(0, *filter);
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::get_test_file;
    use crate::Solution;

    use super::Day03;

    #[test]
    fn test_part_1_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day03 {}.part_1(&test_file), "198");
    }

    #[test]
    fn test_part_1_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day03 {}.part_1(&test_file), "4118544");
    }

    #[test]
    fn test_part_2_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day03 {}.part_2(&test_file), "230");
    }

    #[test]
    fn test_part_2_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day03 {}.part_2(&test_file), "3832770");
    }
}
