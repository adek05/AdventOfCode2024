use crate::utils::read_lines;

fn eval(expected: u64, result: u64, tail: &[u64]) -> bool {
    if tail.is_empty() {
        expected == result
    } else {
        eval(expected, result + tail[0], &tail[1..])
            || eval(expected, result * tail[0], &tail[1..])
            || eval(
                expected,
                format!("{}{}", result, tail[0]).parse::<u64>().unwrap(),
                &tail[1..],
            )
    }
}

pub fn run() {
    let lines: Vec<(u64, Vec<u64>)> = read_lines("in/day7.in")
        .unwrap()
        .map(|x| {
            let_scan!(x.unwrap(); (let expected: u64, ":", [let tail: u64]+ ));
            (expected, tail)
        })
        .collect();

    let res: Vec<u64> = lines
        .iter()
        .map(|(expected, list)| {
            if eval(*expected, list[0], &list[1..]) {
                *expected
            } else {
                0
            }
        })
        .collect();
    println!("Day 7.1: {}", res.iter().sum::<u64>());
}
