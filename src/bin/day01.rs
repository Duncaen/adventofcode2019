use std::fs::File;
use std::io::{prelude::BufRead, BufReader};

fn fuel(mass: i32) -> i32 {
    ((mass / 3) as f64).floor() as i32 - 2
}

fn fuel_rec(mass: i32) -> i32 {
    let y = fuel(mass);
    if y <= 0 {
        return 0;
    }
    y + fuel_rec(y)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("day01")?;
    let f = BufReader::new(f);
    let mut total: i32 = 0;
    let mut total2: i32 = 0;
    for line in f.lines() {
        let f = fuel(line?.parse::<i32>()?);
        total += f;
        total2 += f + fuel_rec(f);
    }
    println!("part1: {}", total);
    println!("part2: {}", total2);
    Ok(())
}

#[test]
fn examples() {
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 654);
    assert_eq!(fuel(100756), 33583);
    assert_eq!(fuel_rec(12), 2);
    assert_eq!(fuel_rec(1969), 966);
    assert_eq!(fuel_rec(100756), 50346);
}
