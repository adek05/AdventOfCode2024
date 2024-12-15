use std::collections::HashSet;
use std::collections::VecDeque;

use crate::utils::get;
use crate::utils::read_lines;

type Point = (i64, i64);
const MOVES: [Point; 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn calculate(visited: &mut HashSet<Point>, map: &[Vec<char>], position: Point) -> i64 {
    let id = map[position.0 as usize][position.1 as usize];

    let mut queue = VecDeque::new();
    queue.push_back((position, 5));

    let mut area = 0;
    let mut perimeter_elem = HashSet::new();
    while let Some((pos, dir)) = queue.pop_front() {
        let next = get(map, pos.0, pos.1, '.');
        if next != id {
            perimeter_elem.insert((dir, pos));
        }

        if next == id {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            area += 1;
            for (i, m) in MOVES.iter().enumerate() {
                queue.push_back(((m.0 + pos.0, m.1 + pos.1), i));
            }
        }
    }

    let mut res = 0;

    {
        let mut items: Vec<Point> = perimeter_elem
            .iter()
            .cloned()
            .filter_map(|(d, point)| if d == 0 { Some(point) } else { None })
            .collect();
        items.sort_by_key(|(x, y)| (*x, *y));
        let mut prev = (-10, -10);
        for cur in items {
            if prev.0 != cur.0 || prev.1 + 1 != cur.1 {
                res += 1;
            }
            prev = cur;
        }
    }
    {
        let mut items: Vec<Point> = perimeter_elem
            .iter()
            .cloned()
            .filter_map(|(d, point)| if d == 1 { Some(point) } else { None })
            .collect();
        items.sort_by_key(|(x, y)| (*x, *y));
        let mut prev = (-10, -10);
        for cur in items {
            if prev.0 != cur.0 || prev.1 + 1 != cur.1 {
                res += 1;
            }
            prev = cur;
        }
    }
    {
        let mut items: Vec<Point> = perimeter_elem
            .iter()
            .cloned()
            .filter_map(|(d, point)| if d == 2 { Some(point) } else { None })
            .collect();
        items.sort_by_key(|(x, y)| (*y, *x));
        let mut prev = (-10, -10);
        for cur in items {
            if prev.1 != cur.1 || prev.0 + 1 != cur.0 {
                res += 1;
            }
            prev = cur;
        }
    }
    {
        let mut items: Vec<Point> = perimeter_elem
            .iter()
            .cloned()
            .filter_map(|(d, point)| if d == 3 { Some(point) } else { None })
            .collect();
        items.sort_by_key(|(x, y)| (*y, *x));
        let mut prev = (-10, -10);
        for cur in items {
            if prev.1 != cur.1 || prev.0 + 1 != cur.0 {
                res += 1;
            }
            prev = cur;
        }
    }

    area * res
}

pub fn run() {
    let lines: Vec<Vec<char>> = read_lines("in/day12.in")
        .unwrap()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let mut visited: HashSet<Point> = HashSet::new();
    let mut res = 0;
    for (i, row) in lines.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if visited.contains(&(i as i64, j as i64)) {
                continue;
            }
            res += calculate(&mut visited, &lines, (i as i64, j as i64));
        }
    }

    println!("Day 12.1: {}", res);
}
