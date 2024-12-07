use crate::utils::read_lines;

pub fn run() {
    let lines: Vec<String> = read_lines("in/day8.in")
        .unwrap()
        .map(|x| x.unwrap())
        .collect();
}