use itertools::Itertools;
use std::{fs, vec};

use crate::y21::Day15;
use aoc::Solution;

fn read_input(input_file: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(input_file).unwrap();
    contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn expand(m: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    fn plus_one(m: &mut Vec<Vec<u32>>) {
        for row in m.iter_mut() {
            for e in row.iter_mut() {
                *e += 1;
                if *e > 9 {
                    *e = 1;
                }
            }
        }
    }

    fn concat(this: &mut Vec<Vec<u32>>, that: &mut Vec<Vec<u32>>, axis: i32) {
        if axis == 0 {
            this.append(that);
        } else if axis == 1 {
            for i in 0..this.len() {
                this[i].append(&mut that[i]);
            }
        } else {
            panic!()
        }
    }

    let mut result = m.clone();
    let mut to_concat = result.clone();
    for _ in 0..4 {
        plus_one(&mut to_concat);
        concat(&mut result, &mut to_concat.clone(), 0);
    }

    let mut to_concat = result.clone();
    for _ in 0..4 {
        plus_one(&mut to_concat);
        concat(&mut result, &mut to_concat.clone(), 1);
    }

    result
}

fn get_neighbors(x: usize, y: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < h - 1 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < w - 1 {
        neighbors.push((x, y + 1));
    }
    neighbors
}

fn dijkstra(risk_levels: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let h = risk_levels.len();
    let w = risk_levels.first().unwrap().len();

    let mut dist: Vec<Vec<Option<u32>>> = vec![vec![None; w]; h];
    let mut queue: Vec<(usize, usize)> = Vec::new();

    dist[0][0] = Some(0);
    queue.push((0, 0));
    while !queue.is_empty() {
        queue.sort_by(|this, that| {
            let this_dist = dist[this.0][this.1].unwrap();
            let that_dist = dist[that.0][that.1].unwrap();
            this_dist.cmp(&that_dist)
        });

        let p = queue.remove(0);

        get_neighbors(p.0, p.1, h, w)
            .into_iter()
            .for_each(|(nx, ny)| {
                let alt = dist[p.0][p.1].unwrap() + risk_levels[nx][ny];
                if let Some(old_dist) = dist[nx][ny] {
                    if alt < old_dist {
                        dist[nx][ny] = Some(alt);
                    }
                } else {
                    dist[nx][ny] = Some(alt);
                    queue.push((nx, ny));
                }
            })
    }

    dist.iter()
        .map(|r| r.iter().map(|e| e.unwrap()).collect_vec())
        .collect_vec()
}

fn a_star_search(risk_levels: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let h = risk_levels.len();
    let w = risk_levels.first().unwrap().len();

    let mut g_score = vec![vec![u32::MAX; w]; h];
    g_score[0][0] = 0;
    let mut f_score = vec![vec![u32::MAX; w]; h];
    f_score[0][0] = (h + w) as u32;

    let mut open_set: Vec<(usize, usize)> = Vec::new();
    open_set.push((0, 0));

    while !open_set.is_empty() {
        open_set.sort_by_key(|(i, j)| f_score[*i][*j]);
        let (x, y) = open_set.remove(0);

        if (x, y) == (h - 1, w - 1) {
            return f_score;
        }

        get_neighbors(x, y, h, w).into_iter().for_each(|(nx, ny)| {
            let tentative_g_score = g_score[x][y] + risk_levels[nx][ny];
            if tentative_g_score < g_score[nx][ny] {
                g_score[nx][ny] = tentative_g_score;
                f_score[nx][ny] = tentative_g_score + (h - 1 - nx) as u32 + (w - 1 - ny) as u32;
                if !open_set.contains(&(nx, ny)) {
                    open_set.push((nx, ny));
                }
            }
        })
    }
    panic!()
}

impl Solution for Day15 {
    fn part_1(&self, input_file: &str) -> String {
        let risk_levels = read_input(input_file);

        let dist = dijkstra(&risk_levels);
        dist.last().unwrap().last().unwrap().to_string()
    }

    fn part_2(&self, input_file: &str) -> String {
        let risk_levels = read_input(input_file);
        let expanded_risk_levels = expand(&risk_levels);

        let dist = a_star_search(&expanded_risk_levels);
        dist.last().unwrap().last().unwrap().to_string()
    }
}
