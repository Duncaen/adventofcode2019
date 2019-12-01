use std::fs::File;
use std::io::{prelude::BufRead, BufReader};

fn fuel(mass: i32) -> i32 {
    ((mass / 3) as f64).floor() as i32 - 2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("day01")?;
    let f = BufReader::new(f);
    let mut total: i32 = 0;
    for line in f.lines() {
        total += fuel(line?.parse::<i32>()?)
    }
    println!("{}", total);
    Ok(())
}

#[test]
fn examples() {
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 654);
    assert_eq!(fuel(100756), 33583);
}
