use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

use crate::y21::Day13;
use aoc::Solution;
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

enum Fold {
    X(u32),
    Y(u32),
}

impl Point {
    fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::X(v) => {
                if self.x > *v {
                    self.x = 2 * v - self.x
                }
            }
            Fold::Y(v) => {
                if self.y > *v {
                    self.y = 2 * v - self.y
                }
            }
        }
    }
}

fn read_input(input_file: &str) -> (Vec<Point>, Vec<Fold>) {
    let contents = fs::read_to_string(input_file).unwrap();
    let mut lines = contents.lines();

    let mut points = Vec::new();
    loop {
        let line = lines.next().unwrap();
        if !line.is_empty() {
            let (x_str, y_str) = line.split(',').collect_tuple().unwrap();
            points.push(Point {
                x: x_str.parse().unwrap(),
                y: y_str.parse().unwrap(),
            });
        } else {
            break;
        }
    }

    let mut folds = Vec::new();
    while let Some(line) = lines.next() {
        let (axis, value) = scan_fmt!(line, "fold along {}={}", char, u32).unwrap();
        let fold = match axis {
            'x' => Fold::X(value),
            'y' => Fold::Y(value),
            _ => panic!(),
        };
        folds.push(fold);
    }
    (points, folds)
}

fn pretty_prints(points: &Vec<Point>) -> String {
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    let point_set: HashSet<Point> = HashSet::from_iter(points.iter().cloned());
    let mut result = String::new();
    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if point_set.contains(&Point { x, y }) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        result.push('\n');
    }
    result
}

impl Solution for Day13 {
    fn part_1(&self, input_file: &str) -> String {
        let (mut points, folds) = read_input(input_file);
        let fold = folds.first().unwrap();
        points.iter_mut().for_each(|point| point.fold(fold));
        points.into_iter().unique().count().to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let (mut points, folds) = read_input(input_file);
        folds
            .iter()
            .for_each(|fold| points.iter_mut().for_each(|point| point.fold(fold)));
        pretty_prints(&points)
    }
}
