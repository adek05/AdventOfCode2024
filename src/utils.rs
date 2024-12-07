use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get<T>(map: &[Vec<T>], x: i64, y: i64, default: T) -> T
where T: Copy {
    if x < 0 || y < 0 {
        return default;
    }
    map.get(x as usize)
        .and_then(|l| l.get(y as usize))
        .copied()
        .unwrap_or(default)
}
