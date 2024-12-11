use std::collections::HashSet;
use std::collections::VecDeque;

use crate::utils::get;
use crate::utils::read_lines;

type Point = (i64, i64);

const MOVES: [Point; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn get_score(p: Point, map: &[Vec<u32>]) -> usize {
    let mut q = VecDeque::new();
    q.push_back((p, 0));
    let mut res: HashSet<Point> = HashSet::new();
    while let Some(cur) = q.pop_front() {
        let value = get(map, cur.0 .0, cur.0 .1, 100);
        if value == 100 {
            continue;
        }

        if value == cur.1 {
            if value == 9 {
                res.insert(cur.0);
                continue;
            }
            for m in MOVES {
                q.push_back(((cur.0 .0 + m.0, cur.0 .1 + m.1), cur.1 + 1));
            }
        }
    }
    return res.len();
}

fn get_score2(p: Point, map: &[Vec<u32>], ) -> usize {
    let mut q = VecDeque::new();
    q.push_back((p, 0));
    let mut res = 0;
    while let Some(cur) = q.pop_front() {
        let value = get(map, cur.0 .0, cur.0 .1, 100);
        if value == 100 {
            continue;
        }

        if value == cur.1 {
            if value == 9 {
                res += 1;
                continue;
            }
            for m in MOVES {
                q.push_back(((cur.0 .0 + m.0, cur.0 .1 + m.1), cur.1 + 1));
            }
        }
    }
    return res;
}

pub fn run() {
    let lines: Vec<Vec<u32>> = read_lines("in/day10small.in")
        .unwrap()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut res = 0;
    let mut res2 = 0;
    for (i, row) in lines.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            res += get_score((i as i64, j as i64), &lines);
            res2 += get_score2((i as i64, j as i64), &lines);
        }
    }

    println!("Day 10.1: {}", res);
    println!("Day 10.2: {}", res2);
}
