use crate::utils::read_lines;

fn is_safe(levels: &[i64]) -> bool {
    let deltas: Vec<i64> = levels.windows(2).map(|chunk| chunk[0] - chunk[1]).collect();
    (deltas.iter().all(|v| v >= &0) || deltas.iter().all(|v| v <= &0))
        && deltas.iter().all(|v| 1 <= v.abs() && v.abs() <= 3)
}

fn is_safeb(levels: &[i64]) -> bool {
    if is_safe(levels) {
        return true;
    }
    for i in 0..levels.len() {
        let mut x = Vec::new();
        x.extend_from_slice(&levels[0..i]);
        x.extend_from_slice(&levels[i + 1..]);
        if is_safe(&x) {
            return true;
        }
    }
    false
}

pub fn run() {
    let lines = read_lines("in/day2.in").unwrap();
    let levels: Vec<Vec<i64>> = lines
        .map(|line| {
            let_scan!(line.unwrap(); ([let levels: i64]+));
            levels
        })
        .collect();

    println!("Day 2.a: {}", levels.iter().filter(|x| is_safe(x)).count());
    println!("Day 2.b: {}", levels.iter().filter(|x| is_safeb(x)).count());
}
