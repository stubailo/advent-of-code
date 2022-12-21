#![allow(dead_code, unused)]

use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct NumberWithId {
    number: i64,
    id: i32,
}

fn main() {
    let start = Instant::now();
    let contents =
        fs::read_to_string("input2.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let encryption_key = 811589153 as i64;
    let mut list_of_numbers: Vec<NumberWithId> = Vec::new();

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        let number = line.parse::<i64>().unwrap();

        list_of_numbers.push(NumberWithId {
            number: number * encryption_key,
            id: list_of_numbers.len() as i32,
        });
    }

    let original_list_of_numbers = list_of_numbers.clone();

    let num_of_numbers_minus_one = (list_of_numbers.len() - 1) as i64;
    let num_of_numbers: i64 = list_of_numbers.len() as i64;

    // println!(
    //     "list: {:?}",
    //     list_of_numbers
    //         .iter()
    //         .map(|x| x.number)
    //         .collect::<Vec<i64>>()
    // );

    // [1, 2, 3, 10, 1, 2, 3, 10, 1, 2, 3, 10 ...]
    // should be between 1 and 2
    // moving 10 -- first, take it out
    // [1, 2, 3, * 1, 2, 3, 1, 2, 3]
    // then take the number it's supposed to move % len - 1, which is 1
    // then add that to its index
    // [1, 2, 3, 1, * 2, 3, 1, 2, 3]
    // [1, 10, 2, 3]

    for _ in (0..10) {
        for n_w_id in &original_list_of_numbers {
            if n_w_id.number == 0 {
                continue;
            }

            let current_index = list_of_numbers
                .iter()
                .position(|&x| x.id == n_w_id.id)
                .unwrap() as i64;

            let n = n_w_id.number;

            let mut new_index = current_index + (n.abs() % num_of_numbers_minus_one) * n.signum();

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

            if n > 0 && new_index < current_index {
                new_index = new_index + 1;
            }

            let _ = list_of_numbers.remove(current_index as usize);
            list_of_numbers.insert(new_index as usize, *n_w_id);

            // println!(
            //     "n: {}, list: {:?}",
            //     n,
            //     list_of_numbers
            //         .iter()
            //         .map(|x| x.number)
            //         .collect::<Vec<i64>>()
            // );
        }
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
    .sum::<i64>();

    println!("Result is: {}", result);

    println!("Time elapsed is: {:?}", start.elapsed());
}
