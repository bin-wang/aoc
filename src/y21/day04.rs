use itertools::Itertools;
use std::fs::File;
use std::io;
use std::io::BufRead;

use crate::y21::Day04;
use aoc::Solution;

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

impl Solution for Day04 {
    fn part_1(&self, input_file: &str) -> String {
        let (draw_order, mut boards) = self.read_input(input_file);
        for number in draw_order {
            boards.iter_mut().for_each(|b| b.mark(number));
            let winning_boards = boards.iter().filter(|b| b.win()).collect_vec();
            if let Some(board) = winning_boards.first() {
                return (board.sum_of_unmarked_numbers() * number as u32).to_string();
            }
        }
        panic!("No winning board found");
    }

    fn part_2(&self, input_file: &str) -> String {
        let (draw_order, mut boards) = self.read_input(input_file);
        for number in draw_order {
            boards.iter_mut().for_each(|b| b.mark(number));
            let mut winning_board_indices = boards
                .iter()
                .enumerate()
                .filter(|t| t.1.win())
                .map(|t| t.0)
                .collect_vec();
            if boards.len() > winning_board_indices.len() {
                winning_board_indices.sort();
                winning_board_indices.reverse();
                for i in winning_board_indices {
                    boards.remove(i);
                }
            } else {
                return (boards.last().unwrap().sum_of_unmarked_numbers() * number as u32)
                    .to_string();
            }
        }
        panic!("No winning boards found");
    }
}

impl Day04 {
    fn read_input(&self, input_file: &str) -> (Vec<u8>, Vec<Board>) {
        let file = File::open(input_file).unwrap();
        let mut lines = io::BufReader::new(file).lines();
        let draw_order: Vec<u8> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let mut boards: Vec<Board> = Vec::new();
        loop {
            if let Some(Ok(_)) = lines.next() {
                let mut numbers: [[u8; 5]; 5] = [[0; 5]; 5];
                for i in 0..5 {
                    let line = lines.next().unwrap().unwrap();
                    line.trim()
                        .split_whitespace()
                        .map(|s| s.parse::<u8>())
                        .enumerate()
                        .for_each(|it| {
                            if let (j, Ok(v)) = it {
                                numbers[i][j] = v;
                            }
                        })
                }
                let marked = [[false; 5]; 5];
                boards.push(Board { numbers, marked });
            } else {
                break;
            }
        }
        (draw_order, boards)
    }
}
