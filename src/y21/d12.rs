use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

use crate::Solution;

use super::Day12;

fn read_input(input_file: &str) -> HashMap<String, Vec<String>> {
    let mut connection_map = HashMap::new();
    let contents = fs::read_to_string(input_file).unwrap();
    contents.lines().for_each(|l| {
        let t: (&str, &str) = l.split('-').collect_tuple().unwrap();
        connection_map
            .entry(t.0.to_string())
            .or_insert_with(Vec::new)
            .push(t.1.to_string());
        connection_map
            .entry(t.1.to_string())
            .or_insert_with(Vec::new)
            .push(t.0.to_string());
    });
    connection_map
}

fn is_all_upper_case(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase())
}

fn count(path: &[String], node: &str) -> usize {
    path.iter().filter(|s| *s == node).count()
}

impl Solution for Day12 {
    fn part_1(&self, input_file: &str) -> String {
        let connection_map = read_input(input_file);

        fn dfs(path: &mut Vec<String>, connection_map: &HashMap<String, Vec<String>>) -> u32 {
            let current_node = path.last().unwrap();
            if current_node == "end" {
                println!("{:?}", path);
                1
            } else {
                let mut counter = 0;
                for next_node in &connection_map[current_node] {
                    if is_all_upper_case(next_node) || !path.contains(next_node) {
                        path.push(next_node.clone());
                        counter += dfs(path, connection_map);
                        path.pop();
                    }
                }
                counter
            }
        }

        let mut path = vec!["start".to_string()];
        dfs(&mut path, &connection_map).to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let connection_map = read_input(input_file);

        fn dfs(
            connection_map: &HashMap<String, Vec<String>>,
            path: &mut Vec<String>,
            dup_allowed: bool,
        ) -> u32 {
            let current_node = path.last().unwrap();
            if current_node == "end" {
                1
            } else {
                let mut counter = 0;
                for next_node in &connection_map[current_node] {
                    if next_node == "start" {
                        continue;
                    } else if is_all_upper_case(next_node) || !path.contains(next_node) {
                        path.push(next_node.clone());
                        counter += dfs(connection_map, path, dup_allowed);
                        path.pop();
                    } else if dup_allowed && count(path, next_node) < 2 {
                        path.push(next_node.clone());
                        counter += dfs(connection_map, path, false);
                        path.pop();
                    }
                }
                counter
            }
        }

        let mut path = vec!["start".to_string()];
        dfs(&connection_map, &mut path, true).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::get_test_file;
    use crate::Solution;

    use super::Day12;

    #[test]
    fn test_part_1_example() {
        let test_file_1 = get_test_file!("example_1.txt");
        assert_eq!(Day12 {}.part_1(&test_file_1), "10");

        let test_file_2 = get_test_file!("example_2.txt");
        assert_eq!(Day12 {}.part_1(&test_file_2), "19");

        let test_file_3 = get_test_file!("example_3.txt");
        assert_eq!(Day12 {}.part_1(&test_file_3), "226");
    }

    #[test]
    fn test_part_1_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day12 {}.part_1(&test_file), "3292");
    }

    #[test]
    fn test_part_2_example() {
        let test_file_1 = get_test_file!("example_1.txt");
        assert_eq!(Day12 {}.part_2(&test_file_1), "36");

        let test_file_2 = get_test_file!("example_2.txt");
        assert_eq!(Day12 {}.part_2(&test_file_2), "103");

        let test_file_3 = get_test_file!("example_3.txt");
        assert_eq!(Day12 {}.part_2(&test_file_3), "3509");
    }

    #[test]
    fn test_part_2_input() {
        let test_file = get_test_file!("input.txt");
        assert_eq!(Day12 {}.part_2(&test_file), "89592");
    }
}
