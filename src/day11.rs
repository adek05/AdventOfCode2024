use std::collections::HashMap;

use crate::utils::get;
use crate::utils::read_lines;

fn blink(pebble: u64) -> Vec<u64> {
    let s = format!("{}", pebble);
    if pebble == 0 {
        vec![1]
    } else if s.len() % 2 == 0 {
        vec![
            s[0..(s.len() / 2)].parse::<u64>().unwrap(),
            s[(s.len() / 2)..].parse::<u64>().unwrap(),
        ]
    } else {
        vec![pebble * 2024]
    }
}

fn blink2(memo: &mut HashMap<(i32, u64), u64>, pebble: u64, depth: i32) -> u64 {
    if depth == 0 {
        return 1;
    }
    if let Some(r) = memo.get(&(depth, pebble)) {
        return *r;
    }
    let pebbles = blink(pebble);
    let r = pebbles.iter().map(|p| blink2(memo, *p, depth - 1)).sum();
    memo.insert((depth, pebble), r);
    return r;
}

pub fn run() {
    let lines: Vec<String> = read_lines("in/day11.in")
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    let pebbles: Vec<u64> = lines[0].split(' ').map(|s| s.parse::<u64>().unwrap()).collect();

    let mut acc = pebbles.clone();
    for x in 0..25 {
        acc = acc.iter().flat_map(|x| blink(*x)).collect();
    }
    println!("Day 11.1: {}", acc.len());

    let mut memo: HashMap<(i32, u64), u64> = HashMap::new();
    println!("Day 11.1: {}", pebbles.iter().map(|p| blink2(&mut memo, *p, 75)).sum::<u64>());
}


