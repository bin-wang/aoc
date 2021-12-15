use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

use crate::solution::Solution;
use crate::y21::Day08;

struct Entry {
    pattern: Vec<String>,
    output: Vec<String>,
}

impl Entry {
    fn new(line: &str) -> Self {
        if let [pattern_raw, output_raw] = line.split(" | ").collect_vec()[..] {
            let pattern = pattern_raw
                .split_whitespace()
                .map(|s| s.to_string())
                .collect_vec();
            let output = output_raw
                .split_whitespace()
                .map(|s| s.to_string())
                .collect_vec();
            Entry { pattern, output }
        } else {
            panic!("Error while parsing line {}", line);
        }
    }

    fn get_output_number(&self) -> u32 {
        let sorted_pattern = self
            .pattern
            .iter()
            .sorted_by_key(|s| s.len())
            .map(|s| s.chars().sorted().collect::<String>())
            .collect_vec();
        let mut display_map: HashMap<&str, char> = HashMap::new();
        display_map.insert(sorted_pattern[0].as_str(), '1');
        display_map.insert(sorted_pattern[1].as_str(), '7');
        display_map.insert(sorted_pattern[2].as_str(), '4');
        display_map.insert(sorted_pattern[9].as_str(), '8');
        for s in &sorted_pattern[3..6] {
            if common_chars(s, sorted_pattern[1].as_str()) == 3 {
                display_map.insert(s, '3');
            } else if common_chars(s, sorted_pattern[2].as_str()) == 3 {
                display_map.insert(s, '5');
            } else {
                display_map.insert(s, '2');
            }
        }
        for s in &sorted_pattern[6..9] {
            if common_chars(s, sorted_pattern[2].as_str()) == 4 {
                display_map.insert(s, '9');
            } else if common_chars(s, sorted_pattern[0].as_str()) == 2 {
                display_map.insert(s, '0');
            } else {
                display_map.insert(s, '6');
            }
        }

        self.output
            .iter()
            .map(|s| {
                display_map
                    .get(s.chars().sorted().collect::<String>().as_str())
                    .unwrap()
            })
            .collect::<String>()
            .parse::<u32>()
            .unwrap()
    }
}

fn read_input(input_file: &str) -> Vec<Entry> {
    let contents = fs::read_to_string(input_file).unwrap();
    contents.lines().map(|l| Entry::new(l)).collect_vec()
}

fn common_chars(a: &str, b: &str) -> u32 {
    *a.chars()
        .counts_by(|c| b.contains(c))
        .get(&true)
        .unwrap_or(&0) as u32
}

impl Solution for Day08 {
    fn part_1(&self, input_file: &str) -> String {
        let records = read_input(input_file);
        records
            .iter()
            .map(|entry| {
                entry
                    .output
                    .iter()
                    .map(|s| match s.len() {
                        2 | 3 | 4 | 7 => 1,
                        _ => 0,
                    })
                    .sum::<u32>()
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let records = read_input(input_file);
        records
            .iter()
            .map(|entry| entry.get_output_number())
            .sum::<u32>()
            .to_string()
    }
}
