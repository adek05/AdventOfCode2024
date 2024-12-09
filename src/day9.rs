use std::fs::File;

use crate::utils::read_lines;

fn get_file_id(idx: i64) -> i64 {
    idx / 2
}

fn is_empty(idx: i64) -> bool {
    idx % 2 == 1
}
#[derive(Clone, Eq, PartialEq)]
enum Block {
    Empty,
    File(usize, usize),
}

fn debug(filesystem: &[Block]) {
    filesystem.iter().for_each(|x| match x {
        Block::File(x, _) => {
            print!("{}", x)
        }
        Block::Empty => {
            print!(".")
        }
    });
    println!("");
}

pub fn run() {
    let lines: Vec<String> = read_lines("in/day9.in")
        .unwrap()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let mut line: Vec<usize> = lines[0]
        .clone()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    let mut filesystem: Vec<Block> = line
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, len)| {
            if is_empty(idx as i64) {
                vec![Block::Empty; len]
            } else {
                vec![Block::File(get_file_id(idx as i64) as usize, len); len]
            }
        })
        .flatten()
        .collect();

    let mut tail_idx = filesystem.len() - 1;
    let mut idx = 0;
    let mut res = 0;
    while idx <= tail_idx {
        match filesystem[idx] {
            Block::File(x, _) => {
                res += x * idx;
                idx += 1;
            }
            Block::Empty => match filesystem[tail_idx] {
                Block::File(x, _) => {
                    res += x * idx;
                    idx += 1;
                    tail_idx -= 1;
                }
                Block::Empty => {
                    tail_idx -= 1;
                }
            },
        }
    }
    println!("Day 9.1: {}", res);

    idx = 0;
    tail_idx = filesystem.len() - 1;
    while idx <= tail_idx {
        match filesystem[tail_idx] {
            Block::Empty => tail_idx -= 1,
            Block::File(id, len) => {
                if tail_idx < len {
                    break;
                }
                for i in 0..tail_idx - len {
                    if filesystem[i..i + len].iter().all(|x| x == &Block::Empty) {
                        for x in 0..len {
                            filesystem[i + x] = Block::File(id, len);
                            filesystem[tail_idx - x] = Block::Empty;
                        }
                        break;
                    }
                }
                tail_idx -= len;
            }
        }
    }
    println!(
        "Day 9.2: {}",
        filesystem
            .iter()
            .enumerate()
            .map(|(pos, block)| {
                match block {
                    Block::Empty => 0,
                    Block::File(x, _) => x * pos,
                }
            })
            .sum::<usize>()
    );
}
