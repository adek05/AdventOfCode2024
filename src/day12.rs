use crate::utils::get;
use crate::utils::read_lines;

pub fn run() {
    let lines: Vec<String> = read_lines("in/day12.in")
        .unwrap()
        .map(|x| x.unwrap())
        .collect();
}