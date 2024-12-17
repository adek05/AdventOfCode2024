use crate::utils::read_lines;

type Point = (i32, i32);
type Velocity = (i32, i32);

const MAX_X: i32 = 101;
const MAX_Y: i32 = 103;
// const MAX_X: i32 = 11;
// const MAX_Y: i32 = 7;

fn simulate(p: &Point, v: &Velocity, steps: i32) -> Point {
    (
        (p.0 + v.0 * steps) % MAX_X,
        (p.1 + v.1*steps) % MAX_Y
    )
}

fn in_quadrant(robot: &Point, start: &Point, end: &Point) -> bool {
    robot.0 >= start.0 && robot.0 < end.0 && robot.1 >= start.1 && robot.1 < end.1
}

fn print(robots: &[Point]) {
    let mut matrix= [[' '; MAX_X as usize]; MAX_Y as usize];
    for (x, y) in robots {
        matrix[*y as usize][*x as usize] = 'x';
    }
    for row in matrix {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

pub fn run() {
    // let robots: Vec<(Point, Velocity)> = read_lines("in/day14small.in")
    let robots: Vec<(Point, Velocity)> = read_lines("in/day14.in")
        .unwrap()
        .map(|x| {
            let_scan!(
                x.unwrap();
                ("p=", let pos_x: i32, ",", let pos_y: i32, " v=", let vel_x: i32, ",", let vel_y: i32)
            );
            ((pos_x, pos_y), ((vel_x + MAX_X) % MAX_X, (vel_y + MAX_Y) % MAX_Y))
        }
        )
        .collect();

    let robots_pos: Vec<Point> = robots.iter().map(|(p, v)| simulate(p, v, 100)).collect();

    const X: i32 = MAX_X;
    const Y: i32 = MAX_Y;

    let q1 = robots_pos.iter().filter(
        |r| in_quadrant(r, &(0, 0), &(X/2 , Y/2))
    ).count();
    let q2 = robots_pos.iter().filter(
        |r| in_quadrant(r, &(X/2 + 1, 0), &(X , Y/2))
    ).count();
    let q3 = robots_pos.iter().filter(
        |r| in_quadrant(r, &(0, Y/2 + 1), &(X/2 , Y))
    ).count();
    let q4 = robots_pos.iter().filter(
        |r| in_quadrant(r, &(X/2 + 1, Y/2 + 1), &(X , Y))
    ).count();
    dbg!(q1, q2, q3, q4);
    // println!("Day 14.1: {}", q1*q2*q3*q4);


    for i in 1..10000 {
        let robots_pos: Vec<Point> = robots.iter().map(|(p, v)| simulate(p, v, i)).collect();
        println!("===== {} =====", i);
        print(&robots_pos);
    }


}
