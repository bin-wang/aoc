use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fs;

use crate::y21::Day05;
use aoc::Solution;

struct Segment {
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
}

trait NewTrait: Iterator + Clone + Sized {}

impl Segment {
    fn to_points(&self, include_diagonal: bool) -> Vec<(u32, u32)> {
        fn flexible_iter(a: u32, b: u32) -> Box<dyn Iterator<Item = u32>> {
            if a < b {
                Box::new(a..b + 1)
            } else {
                Box::new((b..a + 1).rev())
            }
        }

        if self.start_x == self.end_x {
            flexible_iter(self.start_y, self.end_y)
                .map(|y| (self.start_x, y))
                .collect()
        } else if self.start_y == self.end_y {
            flexible_iter(self.start_x, self.end_x)
                .map(|x| (x, self.start_y))
                .collect()
        } else if include_diagonal {
            flexible_iter(self.start_x, self.end_x)
                .zip(flexible_iter(self.start_y, self.end_y))
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Solution for Day05 {
    fn part_1(&self, input_file: &str) -> String {
        let segments = self.read_input(input_file);
        let mut counter = HashMap::new();
        segments.iter().for_each(|segment| {
            segment
                .to_points(false)
                .iter()
                .for_each(|p| *counter.entry(*p).or_insert(0) += 1)
        });
        counter.iter().filter(|it| *(it.1) > 1).count().to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let segments: Vec<Segment> = self.read_input(input_file);
        let mut counter = HashMap::new();
        segments.iter().for_each(|segment| {
            segment
                .to_points(true)
                .iter()
                .for_each(|p| *counter.entry(*p).or_insert(0) += 1)
        });
        counter.iter().filter(|it| *(it.1) > 1).count().to_string()
    }
}

impl Day05 {
    fn read_input(&self, input_file: &str) -> Vec<Segment> {
        let contents = fs::read_to_string(input_file).unwrap();
        contents
            .trim()
            .split("\n")
            .map(|l| {
                let (start_x, start_y, end_x, end_y) =
                    scan_fmt!(l, "{},{} -> {},{}", u32, u32, u32, u32).unwrap();
                Segment {
                    start_x,
                    start_y,
                    end_x,
                    end_y,
                }
            })
            .collect()
    }
}
