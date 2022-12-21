#![allow(dead_code, unused)]

use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct NumberWithId {
    number: i32,
    id: i32,
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut list_of_numbers: Vec<NumberWithId> = Vec::new();

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        let number = line.parse::<i32>().unwrap();

        list_of_numbers.push(NumberWithId {
            number,
            id: list_of_numbers.len() as i32,
        });
    }

    let original_list_of_numbers = list_of_numbers.clone();

    let num_of_numbers_minus_one = (list_of_numbers.len() - 1) as i32;
    let num_of_numbers: i32 = list_of_numbers.len() as i32;

    // println!("list: {:?}", list_of_numbers);

    for n_w_id in original_list_of_numbers {
        if n_w_id.number == 0 {
            continue;
        }

        let current_index = list_of_numbers
            .iter()
            .position(|&x| x.id == n_w_id.id)
            .unwrap() as i32;

        let n = n_w_id.number;

        let mut new_index =
            current_index as i32 + n + (n.abs() / num_of_numbers_minus_one) * n.signum();

        if new_index < 0 {
            loop {
                new_index = new_index + num_of_numbers;
                if new_index > 0 {
                    break;
                }
            }
        } else if new_index >= num_of_numbers {
            loop {
                new_index = new_index - num_of_numbers;
                if new_index < num_of_numbers {
                    break;
                }
            }
        }

        if n < 0 && new_index > current_index {
            new_index = new_index - 1;
        }

        let _ = list_of_numbers.remove(current_index as usize);
        list_of_numbers.insert(new_index as usize, n_w_id);

        // println!("n: {}, list: {:?}", n, list_of_numbers);
    }

    // find index of 0
    let index_of_zero = list_of_numbers.iter().position(|&x| x.number == 0).unwrap();

    let result = [
        index_of_zero + 1000,
        index_of_zero + 2000,
        index_of_zero + 3000,
    ]
    .map(|x| list_of_numbers[x % list_of_numbers.len()].number)
    .iter()
    .sum::<i32>();

    println!("Result is: {}", result);

    println!("Time elapsed is: {:?}", start.elapsed());
}
