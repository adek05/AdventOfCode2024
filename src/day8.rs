use std::collections::{HashSet};

use crate::utils::get;
use crate::utils::read_lines;

pub fn run() {
    let lines: Vec<Vec<char>> = read_lines("in/day8.in")
        .unwrap()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let mut antennas: Vec<(char, (i64, i64))> = Vec::new();

    for (i, x) in lines.iter().enumerate() {
        for (j, c) in x.iter().enumerate() {
            if *c != '.' {
                antennas.push((*c, (i as i64, j as i64)));
            }
        }
    }

    let mut res = HashSet::new();
    for (c, first) in antennas.iter() {
        for (d, second) in antennas.iter() {
            if c != d || first == second {
                continue;
            }
            let dx = second.0 - first.0;
            let dy = second.1 - first.1;
            let mut mul = 0;
            loop {
                let x = second.0 + dx * mul;
                let y = second.1 + dy * mul;
                let c = get(&lines, x, y, ' ');
                if c != ' ' {
                    res.insert((x, y));
                } else {
                    break;
                }
                mul += 1;
            }
        }
    }

    println!("Day 8.1: {}", res.len());
}
