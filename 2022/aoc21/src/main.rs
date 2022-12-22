#![allow(dead_code, unused)]

use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

#[derive(Debug)]
enum Monkey {
    Val(i64),
    Operation {
        input1: String,
        input2: String,
        operation: char,
    },
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut monkeys: HashMap<String, Monkey> = HashMap::new();

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        let parts: Vec<&str> = line.split(": ").collect();

        let monkey_name = parts[0];

        let monkey = match parts[1].parse::<i64>() {
            Ok(val) => Monkey::Val(val),
            Err(_) => {
                let parts: Vec<&str> = parts[1].split(" ").collect();
                Monkey::Operation {
                    input1: parts[0].to_string(),
                    input2: parts[2].to_string(),
                    operation: if monkey_name == "root" {
                        '-'
                    } else {
                        parts[1].chars().next().unwrap()
                    },
                }
            }
        };

        monkeys.insert(monkey_name.to_string(), monkey);
    }

    // Manually found this
    let max_humn = 5000000000000 as i64;
    let mut lower_bound = 0;
    let mut upper_bound = max_humn;

    loop {
        let middle = (lower_bound + upper_bound) / 2;

        monkeys.insert("humn".to_string(), Monkey::Val(middle));

        // print lower, upper, and middle
        // println!("bounds: {} {} {}", lower_bound, middle, upper_bound);

        let mut monkey_values: HashMap<String, i64> = HashMap::new();

        loop {
            let mut all_monkeys_have_values = true;

            for (monkey_name, monkey) in &monkeys {
                if monkey_values.contains_key(monkey_name) {
                    continue;
                }

                let monkey_value = match monkey {
                    Monkey::Val(val) => Some(*val),
                    Monkey::Operation {
                        input1,
                        input2,
                        operation,
                    } => {
                        let input1_value = monkey_values.get(input1);
                        let input2_value = monkey_values.get(input2);

                        if input1_value.is_none() || input2_value.is_none() {
                            None
                        } else {
                            let input1_value = input1_value.unwrap();
                            let input2_value = input2_value.unwrap();

                            let monkey_value = match operation {
                                '*' => input1_value * input2_value,
                                '/' => input1_value / input2_value,
                                '+' => input1_value + input2_value,
                                '-' => input1_value - input2_value,
                                _ => panic!("Unknown operation"),
                            };

                            Some(monkey_value)
                        }
                    }
                };

                if monkey_value.is_some() {
                    monkey_values.insert(monkey_name.to_string(), monkey_value.unwrap());
                } else {
                    all_monkeys_have_values = false;
                }
            }

            if all_monkeys_have_values {
                break;
            }
        }

        let root_val = monkey_values.get("root").unwrap();

        // println!("root_val: {}", root_val);

        if root_val < &0 {
            upper_bound = middle;
        } else if root_val > &0 {
            lower_bound = middle;
        } else {
            println!("Found it: {}", middle);
            break;
        }
    }

    println!("Time elapsed is: {:?}", start.elapsed());
}
