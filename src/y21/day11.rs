use std::collections::{HashSet, VecDeque};
use std::fs;

use itertools::Itertools;

use crate::y21::Day11;
use aoc::Solution;

const LEN: usize = 10;

fn read_input(input_file: &str) -> Vec<Vec<u8>> {
    let contents = fs::read_to_string(input_file).unwrap();
    contents
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect_vec())
        .collect_vec()
}

fn surrounding_coordinates(i: usize, j: usize) -> Box<dyn Iterator<Item = (usize, usize)>> {
    let mut x = vec![i];
    if i > 0 {
        x.push(i - 1);
    }
    if i < LEN - 1 {
        x.push(i + 1);
    }

    let mut y = vec![j];
    if j > 0 {
        y.push(j - 1);
    }
    if j < LEN - 1 {
        y.push(j + 1);
    }

    Box::new(x.into_iter().cartesian_product(y.into_iter()))
}

fn progress(energy_levels: &mut Vec<Vec<u8>>) -> usize {
    let mut queue = VecDeque::new();
    (0..LEN).cartesian_product(0..10).for_each(|(x, y)| {
        energy_levels[x][y] += 1;
        if energy_levels[x][y] > 9 {
            queue.push_back((x, y))
        }
    });

    let mut flashed = HashSet::new();
    while let Some((x, y)) = queue.pop_front() {
        flashed.insert((x, y));
        surrounding_coordinates(x, y).for_each(|(nx, ny)| {
            if energy_levels[nx][ny] <= 9 {
                energy_levels[nx][ny] += 1;
                if energy_levels[nx][ny] > 9 {
                    queue.push_back((nx, ny));
                }
            }
        })
    }

    for p in flashed.iter() {
        energy_levels[p.0][p.1] = 0;
    }

    flashed.len()
}

impl Solution for Day11 {
    fn part_1(&self, input_file: &str) -> String {
        let mut energy_levels = read_input(input_file);
        let mut counter = 0;
        for _ in 0..100 {
            counter += progress(&mut energy_levels);
        }

        counter.to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let mut energy_levels = read_input(input_file);
        let mut it = 0;
        loop {
            it += 1;
            if progress(&mut energy_levels) == LEN * LEN {
                break;
            }
        }
        it.to_string()
    }
}
