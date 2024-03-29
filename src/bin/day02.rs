use std::fs::File;
use std::io::{prelude::BufRead, BufReader};

fn intcode(mut ops: Vec<i32>) -> Option<Vec<i32>> {
    let mut ip: usize = 0;
    loop {
        match ops.get(ip) {
            Some(1) => {
                let (x, y, z) = (
                    *ops.get(ip + 1)? as usize,
                    *ops.get(ip + 2)? as usize,
                    *ops.get(ip + 3)? as usize,
                );
                let (l, r) = (ops[x], ops[y]);
                ops[z] = l + r;
                ip += 4;
            }
            Some(2) => {
                let (x, y, z) = (
                    *ops.get(ip + 1)? as usize,
                    *ops.get(ip + 2)? as usize,
                    *ops.get(ip + 3)? as usize,
                );
                let (l, r) = (ops[x], ops[y]);
                ops[z] = l * r;
                ip += 4;
            }
            Some(99) => {
                break;
            }
            None => break,
            _ => return None,
        }
    }
    Some(ops)
}

fn find_noun_verb(ops: Vec<i32>, output: i32) -> Option<(i32, i32)> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut v = ops.clone();
            v[1] = noun;
            v[2] = verb;
            if let Some(out) = intcode(v) {
                if *out.get(0)? == output {
                    return Some((noun, verb));
                }
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("day02")?;
    let f = BufReader::new(f);
    let mut v = Vec::new();
    for line in f.lines() {
        for i in line?.split(',') {
            v.push(i.parse::<i32>()?)
        }
    }
    // part 1
    v[1] = 12;
    v[2] = 2;
    if let Some(out) = intcode(v.clone()) {
        println!("part1: {}", out[0])
    }
    if let Some((noun, verb)) = find_noun_verb(v.clone(), 19690720) {
        println!("part2: {}", 100 * noun + verb)
    }
    Ok(())
}

#[test]
fn examples() {
    assert_eq!(intcode(vec![1, 0, 0, 0, 99]).unwrap(), vec![2, 0, 0, 0, 99]);
    assert_eq!(intcode(vec![2, 3, 0, 3, 99]).unwrap(), vec![2, 3, 0, 6, 99]);
    assert_eq!(
        intcode(vec![2, 4, 4, 5, 99, 0]).unwrap(),
        vec![2, 4, 4, 5, 99, 9801]
    );
    assert_eq!(
        intcode(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).unwrap(),
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
}
