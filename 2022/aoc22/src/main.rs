#![allow(dead_code, unused)]

use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
enum Step {
    TurnLeft,
    TurnRight,
    Forward(i32),
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut board: HashMap<(i32, i32), char> = HashMap::new();

    let mut y = 0;
    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        for (i, c) in line.chars().enumerate() {
            if c != '#' && c != '.' {
                continue;
            }

            board.insert((y, i as i32), c);
        }

        y += 1;
    }

    for row in (0..12) {
        for col in (0..15) {
            print!("{}", board.get(&(row, col)).unwrap_or(&' '));
        }
        println!();
    }

    let line = iter.next().unwrap();

    let mut directions: Vec<Step> = Vec::new();

    let mut iter = line.chars().peekable();
    loop {
        let char = iter.next();

        if char.is_none() {
            break;
        }

        let char = char.unwrap();

        if char == 'L' {
            directions.push(Step::TurnLeft);
        } else if char == 'R' {
            directions.push(Step::TurnRight);
        } else if char.is_digit(10) {
            let mut num = char.to_digit(10).unwrap() as i32;
            loop {
                let next_char = iter.peek();

                if next_char.is_none() || !next_char.unwrap().is_digit(10) {
                    directions.push(Step::Forward(num));
                    break;
                }

                num = num * 10 + iter.next().unwrap().to_digit(10).unwrap() as i32;
            }
        }
    }

    // done parsing

    println!("Time elapsed is: {:?}", start.elapsed());
}
