#![allow(dead_code, unused)]

use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Digit {
    Zero,
    One,
    Two,
    Minus,
    MinusMinus,
}

#[derive(Debug)]
struct SNAFUNumber {
    digits: Vec<Digit>,
}

impl SNAFUNumber {
    fn to_decimal(&self) -> i64 {
        let mut num = 0;
        let mut place = 1 as i64;
        for digit in self.digits.iter().rev() {
            match digit {
                Digit::Zero => {
                    num += 0 * place;
                }
                Digit::One => {
                    num += 1 * place;
                }
                Digit::Two => {
                    num += 2 * place;
                }
                Digit::Minus => {
                    num += -1 * place;
                }
                Digit::MinusMinus => {
                    num += -2 * place;
                }
            }

            place *= 5;
        }

        return num;
    }

    fn from_decimal(num: i64) -> SNAFUNumber {
        let mut digits: Vec<Digit> = Vec::new();
        let mut num = num;

        while num != 0 {
            let digit = num % 5;

            match digit {
                0 => digits.push(Digit::Zero),
                1 => digits.push(Digit::One),
                2 => digits.push(Digit::Two),
                3 => {
                    digits.push(Digit::MinusMinus);
                    num += 5;
                }
                4 => {
                    digits.push(Digit::Minus);
                    num += 5;
                }
                _ => panic!("Unknown digit"),
            }

            num /= 5;
        }

        digits.reverse();

        return SNAFUNumber { digits };
    }

    fn to_string(&self) -> String {
        let mut s = String::new();

        for digit in self.digits.iter() {
            match digit {
                Digit::Zero => s.push('0'),
                Digit::One => s.push('1'),
                Digit::Two => s.push('2'),
                Digit::Minus => s.push('-'),
                Digit::MinusMinus => s.push('='),
            }
        }

        return s;
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut numbers: Vec<SNAFUNumber> = Vec::new();

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        let mut digits: Vec<Digit> = Vec::new();

        for c in line.chars() {
            match c {
                '0' => digits.push(Digit::Zero),
                '1' => digits.push(Digit::One),
                '2' => digits.push(Digit::Two),
                '-' => digits.push(Digit::Minus),
                '=' => digits.push(Digit::MinusMinus),
                _ => panic!("Unknown character"),
            }
        }

        numbers.push(SNAFUNumber { digits });
    }

    let mut sum = 0;
    for number in numbers.iter() {
        sum += number.to_decimal();
    }

    println!("Sum is: {}", SNAFUNumber::from_decimal(sum).to_string());

    println!("Time elapsed is: {:?}", start.elapsed());
}
