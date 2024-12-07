use crate::utils::read_lines;
use regex::Regex;

pub fn run() {
    let lines = read_lines("in/day3.in").unwrap();

    let mut enabled = true;

    let res: i64 = lines
        .map(|line| {
            Regex::new(r"(mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\))")
                .unwrap()
                .find_iter(&line.unwrap())
                .map(|x| {
                    if x.as_str() == "do()" {
                        enabled = true;
                        return 0;
                    }
                    if x.as_str() == "don't()" {
                        enabled = false;
                        return 0;
                    }
                    if !enabled {
                        return 0;
                    }
                    let_scan!(x.as_str(); ("mul(", let a: i64, ",", let b: i64, ")"));
                    a * b
                })
                .sum::<i64>()
        })
        .sum();

    println!("Day 3.a: {}", res);
}
