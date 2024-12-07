use std::cmp::Ordering;

use crate::utils::read_lines;

fn is_correct(rules: &Vec<(i32, i32)>, update: &[i32]) -> bool {
    for r in rules {
        let pos_a = update.iter().position(|x| x == &r.0);
        let pos_b = update.iter().position(|x| x == &r.1);
        if let Some(a) = pos_a {
            if let Some(b) = pos_b {
                if a > b {
                    return false;
                }
            }
        }
    }
    true
}

pub fn run() {
    let lines: Vec<String> = read_lines("in/day5.in")
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    let mut rules: Vec<(i32, i32)> = vec![];

    let mut it = lines.iter();
    while let Some(x) = it.next() {
        if x.is_empty() {
            break;
        }
        let_scan!(x; (let a: i32, "|", let b: i32));
        rules.push((a, b));
    }

    let mut res = 0;
    let mut res2 = 0;
    while let Some(l) = it.next() {
        let_scan!(l; ([let val2: i32],+));
        let mut val = val2;
        if is_correct(&rules, &val) {
            res += val[val.len() / 2];
        } else {
            val.sort_by(|a, b| {
                if rules.iter().any(|x| *x == (*a, *b)) {
                    Ordering::Less
                } else if rules.iter().any(|x| *x == (*b, *a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            res2 += val[val.len() / 2];
        }
    }

    println!("Day 5.1: {}", res);
    println!("Day 5.2: {}", res2);
}
