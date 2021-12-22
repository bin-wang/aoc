use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fs;

use crate::solution::Solution;
use crate::y21::Day14;

fn read_input(input_file: &str) -> (String, HashMap<(char, char), char>) {
    let contents = fs::read_to_string(input_file).unwrap();
    let mut lines = contents.lines();

    let template = lines.next().unwrap().to_string();
    lines.next();
    let rules = lines
        .map(|l| {
            let (pattern, insert) = scan_fmt!(l, "{} -> {}", String, char).unwrap();
            let (first, second) = pattern.chars().collect_tuple().unwrap();
            ((first, second), insert)
        })
        .collect();
    (template, rules)
}

fn step(original: &String, rules: &HashMap<(char, char), char>) -> String {
    let original_vec = original.chars().collect_vec();
    let mut result = String::new();
    for i in 0..original_vec.len() {
        result.push(original_vec[i]);
        if i < original_vec.len() - 1 {
            if let Some(insert) = rules.get(&(original_vec[i], original_vec[i + 1])) {
                result.push(*insert);
            }
        }
    }
    result
}

impl Solution for Day14 {
    fn part_1(&self, input_file: &str) -> String {
        let (template, rules) = read_input(input_file);
        let mut result = template;
        for _ in 0..10 {
            result = step(&result, &rules);
        }
        let frequency = result.chars().counts_by(|c| c);
        let sorted_frequency = frequency.iter().sorted_by_key(|p| p.1).collect_vec();
        let most = sorted_frequency.last().unwrap().1;
        let least = sorted_frequency.first().unwrap().1;
        (most - least).to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let (template, rules) = read_input(input_file);
        let mut cache: [HashMap<(char, char), HashMap<char, usize>>; 2] =
            [HashMap::new(), HashMap::new()];
        for (pattern, insert) in rules.iter() {
            cache[1].insert(*pattern, HashMap::from([(*insert, 1)]));
        }

        for i in 2..41 {
            println!("{}", i);
            let current_index = i % 2;
            let previous_index = (i - 1) % 2;
            for (pattern, insert) in rules.iter() {
                let mut frequency = HashMap::from([(*insert, 1)]);
                if let Some(left) = cache[previous_index].get(&(pattern.0, *insert)) {
                    for (k, v) in left.iter() {
                        *frequency.entry(*k).or_insert(0) += v;
                    }
                }

                if let Some(right) = cache[previous_index].get(&(*insert, pattern.1)) {
                    for (k, v) in right.iter() {
                        *frequency.entry(*k).or_insert(0) += v;
                    }
                }
                cache[current_index].insert(*pattern, frequency);
            }
        }

        let mut final_frequency = template.chars().counts();
        let template_vec = template.chars().collect_vec();
        for i in 0..template_vec.len() {
            if i < template_vec.len() - 1 {
                if let Some(f) = cache[0].get(&(template_vec[i], template_vec[i + 1])) {
                    for (k, v) in f.iter() {
                        *final_frequency.entry(*k).or_insert(0) += v;
                    }
                }
            }
        }

        let sorted_frequency = final_frequency.iter().sorted_by_key(|p| p.1).collect_vec();
        let most = sorted_frequency.last().unwrap().1;
        let least = sorted_frequency.first().unwrap().1;
        (most - least).to_string()
    }
}
