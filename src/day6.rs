use std::{collections::HashSet, hash::Hash};

use crate::utils::read_lines;

fn get(map: &[Vec<char>], x: i64, y: i64) -> char {
    if x < 0 || y < 0 {
        return '0';
    }
    map.get(x as usize)
        .map(|l| l.get(y as usize))
        .flatten()
        .map(|c| *c as char)
        .unwrap_or('0')
}

fn has_loop(lines: &[Vec<char>], s: usize, p: (i64, i64)) -> bool {
    // let mut visited = v.clone();
    let mut visited = HashSet::new();
    let MOVES: Vec<(i64, i64)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut step_idx = s;
    let mut pos = p;

    loop {
        if visited.contains(&(pos, step_idx)) {
            return true;
        }
        visited.insert((pos, step_idx));
        let new_pos = (pos.0 + MOVES[step_idx].0, pos.1 + MOVES[step_idx].1);
        let c = get(&lines, new_pos.0, new_pos.1);
        if c == '#' {
            step_idx = (step_idx + 1) % MOVES.len();
            continue;
        }
        if c == '0' {
            return false;
        }
        pos = new_pos;
    }
}

pub fn run() {
    // let mut lines: Vec<Vec<char>> = read_lines("in/day6small.in")
    let mut lines: Vec<Vec<char>> = read_lines("in/day6.in")
        .unwrap()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let mut pos: (i64, i64) = (0, 0);
    let mut step_idx = 0;
    let MOVES: Vec<(i64, i64)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] as char == '^' {
                pos = (i as i64, j as i64);
            }
        }
    }

    let mut visited = HashSet::new();
    let mut res: HashSet<(i64, i64)> = HashSet::new();
    loop {
        let new_pos = (pos.0 + MOVES[step_idx].0, pos.1 + MOVES[step_idx].1);
        let c = get(&lines, new_pos.0, new_pos.1);
        visited.insert(pos);
        if c == '0' {
            break;
        }
        if c == '#' {
            step_idx = (step_idx + 1) % MOVES.len();
            continue;
        }
        if !visited.contains(&new_pos) {
            lines[new_pos.0 as usize][new_pos.1 as usize] = '#';
            if has_loop(&lines, step_idx % MOVES.len(), pos) {
                res.insert(new_pos);
            }
            lines[new_pos.0 as usize][new_pos.1 as usize] = '.';
        }
        pos = new_pos;
    }

    println!("Day6.1: {}", visited.len());
    println!("Day6.2: {}", res.len());

}
