use scan_fmt::scan_fmt;
use std::fs;

use crate::y21::Day02;
use aoc::Solution;

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Solution for Day02 {
    fn part_1(&self, input_file: &str) -> String {
        let commands = self.read_input(input_file);
        let mut horizontal_pos = 0;
        let mut depth = 0;
        for command in commands {
            match command {
                Command::Forward(x) => horizontal_pos += x,
                Command::Up(x) => depth -= x,
                Command::Down(x) => depth += x,
            }
        }
        (horizontal_pos * depth).to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let commands = self.read_input(input_file);
        let mut horizontal_pos = 0;
        let mut depth = 0;
        let mut aim = 0;
        for command in commands {
            match command {
                Command::Up(x) => aim -= x,
                Command::Down(x) => aim += x,
                Command::Forward(x) => {
                    horizontal_pos += x;
                    depth += aim * x;
                }
            }
        }
        (horizontal_pos * depth).to_string()
    }
}

impl Day02 {
    fn read_input(&self, input_file: &str) -> Vec<Command> {
        let contents = fs::read_to_string(input_file).expect("Could not read file");
        contents
            .trim()
            .lines()
            .map(|l| {
                let (command, x) = scan_fmt!(l, "{} {}", String, u32).unwrap();
                match command.as_str() {
                    "forward" => Command::Forward(x),
                    "up" => Command::Up(x),
                    "down" => Command::Down(x),
                    _ => panic!("Unexpected command {}", command),
                }
            })
            .collect()
    }
}
