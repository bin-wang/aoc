use std::fs;

use itertools::Itertools;

use crate::Solution;

use super::Day04;

struct Board {
    numbers: [[u8; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    fn mark(&mut self, number: u8) {
        for (i, j) in (0..5).cartesian_product(0..5) {
            if self.numbers[i][j] == number {
                self.marked[i][j] = true;
            }
        }
    }

    fn win(&self) -> bool {
        let has_marked_row = (0..5).any(|i| (0..5).all(|j| self.marked[i][j]));
        let has_marked_col = (0..5).any(|i| (0..5).all(|j| self.marked[j][i]));
        has_marked_row || has_marked_col
    }

    fn sum_of_unmarked_numbers(&self) -> u32 {
        let mut sum: u32 = 0;
        for (i, j) in (0..5).cartesian_product(0..5) {
            if !self.marked[i][j] {
                sum += self.numbers[i][j] as u32;
            }
        }
        sum
    }
}

fn read_input(input_file: &str) -> (Vec<u8>, Vec<Board>) {
    let contents = fs::read_to_string(input_file).unwrap();
    let mut lines = contents.lines();
    let draw_order: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut boards: Vec<Board> = Vec::new();
    while let Some(_) = lines.next() {
        let mut numbers: [[u8; 5]; 5] = [[0; 5]; 5];
        for row in &mut numbers {
            let line = lines.next().unwrap();
            let v = line
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .collect_vec();
            *row = <[u8; 5]>::try_from(&v[..]).unwrap();
        }
        let marked = [[false; 5]; 5];
        boards.push(Board { numbers, marked });
    }
    (draw_order, boards)
}

impl Solution for Day04 {
    fn part_1(&self, input_file: &str) -> String {
        let (draw_order, mut boards) = read_input(input_file);
        for number in draw_order {
            boards.iter_mut().for_each(|b| b.mark(number));
            let winning_boards = boards.iter().filter(|b| b.win()).collect_vec();
            if let Some(board) = winning_boards.first() {
                return (board.sum_of_unmarked_numbers() * number as u32).to_string();
            }
        }
        panic!("No winning board found!")
    }

    fn part_2(&self, input_file: &str) -> String {
        let (draw_order, mut boards) = read_input(input_file);
        for number in draw_order {
            boards.iter_mut().for_each(|b| b.mark(number));
            let mut winning_board_indices = boards
                .iter()
                .enumerate()
                .filter(|t| t.1.win())
                .map(|t| t.0)
                .collect_vec();
            if boards.len() > winning_board_indices.len() {
                winning_board_indices.sort_unstable();
                winning_board_indices.reverse();
                for i in winning_board_indices {
                    boards.remove(i);
                }
            } else {
                return (boards.last().unwrap().sum_of_unmarked_numbers() * number as u32)
                    .to_string();
            }
        }
        panic!("Not all boards won!")
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::get_test_file;
    use crate::Solution;

    use super::Day04;

    #[test]
    fn test_part_1_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day04 {}.part_1(&test_file), "4512");
    }

    #[test]
    fn test_part_1_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day04 {}.part_1(&test_file), "29440");
    }

    #[test]
    fn test_part_2_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day04 {}.part_2(&test_file), "1924");
    }

    #[test]
    fn test_part_2_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day04 {}.part_2(&test_file), "13884");
    }
}
