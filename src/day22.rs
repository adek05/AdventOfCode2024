use crate::utils::read_lines;

pub fn run() {
    let lines: Vec<String> = read_lines("in/dayXX.in")
        .unwrap()
        .map(|x| x.unwrap())
        .collect();
}