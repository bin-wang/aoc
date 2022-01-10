use std::fs;

use scan_fmt::scan_fmt;

use crate::Solution;

use super::Day02;

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Solution for Day02 {
    fn part_1(&self, input_file: &str) -> String {
        let commands = read_input(input_file);
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
        let commands = read_input(input_file);
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

fn read_input(input_file: &str) -> Vec<Command> {
    fn parse_line(line: &str) -> Command {
        let (command, x) = scan_fmt!(line, "{} {}", String, u32).unwrap();
        match command.as_str() {
            "forward" => Command::Forward(x),
            "up" => Command::Up(x),
            "down" => Command::Down(x),
            _ => panic!("Unexpected command: {}", command),
        }
    }

    let contents = fs::read_to_string(input_file).unwrap();
    contents.lines().map(parse_line).collect()
}

#[cfg(test)]
mod tests {
    use crate::test_util::get_test_file;
    use crate::Solution;

    use super::Day02;

    #[test]
    fn test_part_1_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day02 {}.part_1(&test_file), "150");
    }

    #[test]
    fn test_part_1_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day02 {}.part_1(&test_file), "1924923");
    }

    #[test]
    fn test_part_2_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day02 {}.part_2(&test_file), "900");
    }

    #[test]
    fn test_part_2_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day02 {}.part_2(&test_file), "1982495697");
    }
}
