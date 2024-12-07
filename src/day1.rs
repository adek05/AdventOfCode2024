use crate::utils::read_lines;

pub fn run() {
    let lines = read_lines("in/day1.in").unwrap();
    let mut list_a: Vec<i64> = vec![];
    let mut list_b: Vec<i64> = vec![];

    lines.for_each(|line| if let Ok(l) = line {
        let_scan!(l; (let first: i64, let second: i64));
        list_a.push(first);
        list_b.push(second);

    });
    list_a.sort();
    list_b.sort();
    println!("Day 1.a: {}",
    list_a.iter().zip(list_b.iter()).map(
        |(a,b)| (a-b).abs()

    ).sum::<i64>());

    let mut res = 0;
    list_a.iter().for_each(|elem| {
        res += list_b.iter().filter(|x| x == &elem).count() as i64 * elem
    });
    println!("Day 1.b: {}", res);
}
