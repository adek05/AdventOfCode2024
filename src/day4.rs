use crate::utils::read_lines;

fn get(table: &[String], x: i32, y: i32) -> char {
    if x >= 0 && x < table.len() as i32 && y >= 0 && y < table[x as usize].len() as i32 {
        return table[x as usize].as_bytes()[y as usize] as char;
    }
    '.'
}

pub fn run() {
    let lines: Vec<String> = read_lines("in/day4.in")
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    let moves = [
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    let mut res = 0;
    for x in 0..lines.len() {
        for y in 0..lines[x].len() {
            for m in moves {
                if get(&lines, x as i32, y as i32) == 'X'
                    && get(&lines, x as i32 + m.0, y as i32 + m.1) == 'M'
                    && get(&lines, x as i32 + 2 * m.0, y as i32 + 2 * m.1) == 'A'
                    && get(&lines, x as i32 + 3 * m.0, y as i32 + 3 * m.1) == 'S'
                {
                    res += 1;
                }
            }
        }
    }

    let mut res2 = 0;
    for x in 0..lines.len() {
        for y in 0..lines[x].len() {
            if get(&lines, x as i32, y as i32) == 'A' {
                let mut one = vec![
                    get(&lines, x as i32 + 1, y as i32 + 1),
                    get(&lines, x as i32 - 1, y as i32 - 1),
                ];
                let mut two = vec![
                    get(&lines, x as i32 - 1, y as i32 + 1),
                    get(&lines, x as i32 + 1, y as i32 - 1),
                ];
                one.sort();
                two.sort();
                if one == vec!['M', 'S'] && two == vec!['M', 'S'] {
                    res2 += 1;
                    continue;
                }
            }
        }
    }
    println!("Day 4.1: {}", res);
    println!("Day 4.2: {}", res2);
}
