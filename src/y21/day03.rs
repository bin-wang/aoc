use std::fs;

use crate::y21::Day03;
use aoc::Solution;

enum BitCriteria {
    MostCommon,
    LeastCommon,
}

impl Solution for Day03 {
    fn part_1(&self, input_file: &str) -> String {
        let input = self.read_input(input_file);
        let digit_len = input.first().unwrap().len();
        let mut counter: Vec<u32> = vec![0; digit_len];
        for x in &input {
            let b = x.as_str().as_bytes();
            for i in 0..digit_len {
                counter[i] += (b[i] - '0' as u8) as u32;
            }
        }
        let mut gamma_binary = String::new();
        let mut epsilon_binary = String::new();
        let list_len = input.len();
        for d in counter {
            if (2 * d as usize) > list_len {
                gamma_binary.push('1');
                epsilon_binary.push('0');
            } else {
                gamma_binary.push('0');
                epsilon_binary.push('1');
            }
        }
        let gamma_rate = u32::from_str_radix(gamma_binary.as_str(), 2).unwrap();
        let epsilon_rate = u32::from_str_radix(epsilon_binary.as_str(), 2).unwrap();
        (gamma_rate * epsilon_rate).to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let input = self.read_input(input_file);
        let oxygen_binary = self.part_2_helper(&input, BitCriteria::MostCommon);
        let oxygen_rating = u32::from_str_radix(oxygen_binary.as_str(), 2).unwrap();
        let co2_binary = self.part_2_helper(&input, BitCriteria::LeastCommon);
        let co2_rating = u32::from_str_radix(co2_binary.as_str(), 2).unwrap();
        (oxygen_rating * co2_rating).to_string()
    }
}

impl Day03 {
    fn read_input(&self, input_file: &str) -> Vec<String> {
        let contents = fs::read_to_string(input_file).expect("Could not read file");
        contents.trim().split("\n").map(|s| s.to_string()).collect()
    }

    fn part_2_helper(&self, number_binary_reprs: &Vec<String>, criteria: BitCriteria) -> String {
        if number_binary_reprs.len() == 1 {
            number_binary_reprs.first().unwrap().to_string()
        } else if number_binary_reprs.len() == 0 {
            String::new()
        } else {
            let mut count: u32 = 0;
            for x in number_binary_reprs {
                count += (x.as_bytes()[0] - '0' as u8) as u32;
            }
            let filter = match criteria {
                BitCriteria::MostCommon => {
                    if (2 * count as usize) >= number_binary_reprs.len() {
                        '1'
                    } else {
                        '0'
                    }
                }
                BitCriteria::LeastCommon => {
                    if (2 * count as usize) < number_binary_reprs.len() {
                        '1'
                    } else {
                        '0'
                    }
                }
            };
            let filtered: Vec<String> = number_binary_reprs
                .iter()
                .filter(|s| s.as_bytes()[0] == filter as u8)
                .map(|s| {
                    let mut s = s.to_string();
                    s.remove(0);
                    s
                })
                .collect();
            let mut ans = self.part_2_helper(&filtered, criteria);
            ans.insert(0, filter);
            ans
        }
    }
}
