use std::collections::{HashMap, HashSet};
use std::fs;

use itertools::Itertools;

use crate::Solution;

use super::Day09;

fn read_input(input_file: &str) -> Vec<Vec<u8>> {
    let contents = fs::read_to_string(input_file).unwrap();
    contents
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
        .collect_vec()
}

fn surrounding_coordinates(
    i: usize,
    j: usize,
    h: usize,
    w: usize,
) -> Box<dyn Iterator<Item = (usize, usize)>> {
    let mut coordinates = Vec::new();
    if i > 0 {
        coordinates.push((i - 1, j))
    }
    if i < h - 1 {
        coordinates.push((i + 1, j));
    }
    if j > 0 {
        coordinates.push((i, j - 1));
    }
    if j < w - 1 {
        coordinates.push((i, j + 1));
    }
    Box::new(coordinates.into_iter())
}

fn find_low_points<'a>(heights: &'a [Vec<u8>]) -> Box<dyn Iterator<Item = (usize, usize)> + 'a> {
    let h = heights.len();
    let w = heights[0].len();
    let it = (0..h).cartesian_product(0..w).filter(move |(i, j)| {
        surrounding_coordinates(*i, *j, h, w).all(|(x, y)| heights[*i][*j] < heights[x][y])
    });
    Box::new(it)
}

fn get_basin_size(heights: &[Vec<u8>], i: usize, j: usize) -> u32 {
    let h = heights.len();
    let w = heights[0].len();
    let mut queue = Vec::new();
    let mut visited = HashSet::new();
    queue.push((i, j));
    visited.insert((i, j));
    let mut size = 0;
    while let Some((x, y)) = queue.pop() {
        size += 1;
        for (nx, ny) in surrounding_coordinates(x, y, h, w) {
            if heights[nx][ny] == 9 {
                continue;
            }
            if !visited.contains(&(nx, ny)) && heights[nx][ny] > heights[x][y] {
                queue.push((nx, ny));
                visited.insert((nx, ny));
            }
        }
    }
    size
}

impl Solution for Day09 {
    fn part_1(&self, input_file: &str) -> String {
        let heights = read_input(input_file);
        find_low_points(&heights)
            .map(|(i, j)| heights[i][j] as u32 + 1)
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let heights = read_input(input_file);
        let mut size_map: HashMap<(usize, usize), u32> = HashMap::new();
        find_low_points(&heights).for_each(|(i, j)| {
            let size = get_basin_size(&heights, i, j);
            size_map.insert((i, j), size);
        });

        let sizes = size_map.values().sorted().rev().take(3).collect_vec();
        let mut product = 1;
        for size in sizes {
            product *= size;
        }
        product.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::get_test_file;
    use crate::Solution;

    use super::Day09;

    #[test]
    fn test_part_1_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day09 {}.part_1(&test_file), "15");
    }

    #[test]
    fn test_part_1_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day09 {}.part_1(&test_file), "562");
    }

    #[test]
    fn test_part_2_example() {
        let test_file = get_test_file!("example.txt");
        assert_eq!(Day09 {}.part_2(&test_file), "1134");
    }

    #[test]
    fn test_part_2_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day09 {}.part_2(&test_file), "1076922");
    }
}
