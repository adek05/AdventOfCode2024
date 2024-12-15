use std::cmp::min;

use crate::utils::read_lines;

const A_COST: i64 = 3;
const D_PRIZE: i64 = 10000000000000;

#[derive(Debug)]
struct Equation {
    pub a: (i64, i64),
    pub b: (i64, i64),
    pub prize: (i64, i64),
}

fn solve(eq: &Equation) -> i64 {
    let a = eq.a;
    let b = eq.b;
    let prize = eq.prize;

    let mut min_cost = 1 << 60;
    for i in 1..(prize.0 / a.0 + 1) {
        let dx = prize.0 - a.0 * i;
        let dy = prize.1 - a.1 * i;

        if dx % b.0 == 0 && dy % b.1 == 0 && dx / b.0 == dy / b.1 {
            min_cost = min(i * A_COST + dx / b.0, min_cost);
            println!("Match on i={}", i);
        }
    }
    if min_cost == 1 << 60 {
        0
    } else {
        min_cost
    }
}

fn solve2(eq: &Equation) -> i64 {
    let a = eq.a;
    let b = eq.b;
    let p = eq.prize;

    let la = (a.0 * b.1 - a.1 * b.0);
    let ra = p.0 * b.1 - p.1 * b.0;

    let lb = (b.0 * a.1 - b.1 * a.0);
    let rb = p.0 * a.1 - p.1 * a.0;

    if ra % la == 0 {
        let button_a = ra / la;
        if rb % lb == 0 {
            let button_b = rb / lb;
            return 3 * button_a + button_b;
            println!("Hit: {} {}", button_a, button_b);
        }
    }

    return 0;
}

pub fn run() {
    let lines: Vec<String> = read_lines("in/day13.in")
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    let eqs: Vec<Equation> = lines
        .split(|x| x.is_empty())
        .map(|split| {
            let_scan!(&split[0]; ("Button A: X+", let adx: i64, ", Y+", let ady: i64));
            let_scan!(&split[1]; ("Button B: X+", let bdx: i64, ", Y+", let bdy: i64));
            let_scan!(&split[2]; ("Prize: X=", let px: i64, ", Y=", let py: i64));
            Equation {
                a: (adx, ady),
                b: (bdx, bdy),
                prize: (px + D_PRIZE, py + D_PRIZE),
            }
        })
        .collect();

    let res: i64 = eqs.iter().map(|e| solve2(e)).sum();
    println!("{:?}", res);
}
