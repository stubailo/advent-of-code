use std::collections::HashMap;
use std::fs;
use std::time::Instant;

struct Monkey {
    items: Vec<u64>,
    operation: String,
    test_divisible_by: u64,
    next_monkey_if_true: usize,
    next_monkey_if_false: usize,
}

fn main() {
    let start = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut iter = contents.lines();

    // Start by parsing input, of the form:
    // Monkey 0:
    //   Starting items: 93, 98
    //   Operation: new = old * 17
    //   Test: divisible by 19
    //     If true: throw to monkey 5
    //     If false: throw to monkey 3

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_index_to_num_inspected: HashMap<usize, u64> = HashMap::new();

    'outer: loop {
        let mut current_monkey = Monkey {
            items: Vec::new(),
            operation: String::new(),
            test_divisible_by: 0,
            next_monkey_if_true: 0,
            next_monkey_if_false: 0,
        };

        loop {
            let line = iter.next();

            if line.is_none() {
                monkeys.push(current_monkey);
                break 'outer;
            }

            let line = line.unwrap();

            if line.is_empty() {
                monkeys.push(current_monkey);
                continue 'outer;
            } else if line.starts_with("Monkey") {
                // do nothing
            } else if line.trim().starts_with("Starting") {
                line.split(": ").nth(1).unwrap().split(", ").for_each(|x| {
                    current_monkey.items.push(x.parse::<u64>().unwrap());
                });
            } else if line.trim().starts_with("Operation") {
                current_monkey.operation = line.split("= ").last().unwrap().to_string();
            } else if line.trim().starts_with("Test") {
                current_monkey.test_divisible_by =
                    line.split(" ").last().unwrap().parse::<u64>().unwrap();
            } else if line.trim().starts_with("If true") {
                current_monkey.next_monkey_if_true =
                    line.split(" ").last().unwrap().parse::<usize>().unwrap();
            } else if line.trim().starts_with("If false") {
                current_monkey.next_monkey_if_false =
                    line.split(" ").last().unwrap().parse::<usize>().unwrap();
            }
        }
    }

    // do 20 rounds of simulation
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];

            // First item in the tuple is the monkey to give to, second is the worry level to give
            let mut to_throw: Vec<(usize, u64)> = Vec::new();

            for item in &monkey.items {
                monkey_index_to_num_inspected
                    .entry(i)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);

                // apply operation
                let mut operation_tokens = monkey.operation.split_whitespace();

                let operand_1 = operation_tokens.next().unwrap();
                let operator = operation_tokens.next().unwrap();
                let operand_2 = operation_tokens.next().unwrap();

                let operand_1_value = if operand_1 == "old" {
                    *item
                } else {
                    operand_1.parse::<u64>().unwrap()
                };

                let operand_2_value = if operand_2 == "old" {
                    *item
                } else {
                    operand_2.parse::<u64>().unwrap()
                };

                let new_item_intermediate = match operator {
                    "*" => operand_1_value * operand_2_value,
                    "+" => operand_1_value + operand_2_value,
                    "-" => operand_1_value - operand_2_value,
                    "/" => operand_1_value / operand_2_value,
                    _ => panic!("Unknown operator"),
                };

                let new_item = new_item_intermediate % 9699690;

                // now we determine which monkey to give to
                let next_monkey = if new_item % monkey.test_divisible_by == 0 {
                    monkey.next_monkey_if_true
                } else {
                    monkey.next_monkey_if_false
                };

                to_throw.push((next_monkey, new_item));
            }

            monkey.items = Vec::new();

            for (monkey_to_throw_to, item) in to_throw {
                monkeys[monkey_to_throw_to].items.push(item);
            }
        }
    }

    // get values of monkey_index_to_num_inspected
    let mut monkey_index_to_num_inspected_values: Vec<u64> = Vec::new();
    for (_, v) in monkey_index_to_num_inspected {
        monkey_index_to_num_inspected_values.push(v);
    }

    // sort them
    monkey_index_to_num_inspected_values.sort();
    monkey_index_to_num_inspected_values.reverse();

    // multiply the first 2
    let answer = monkey_index_to_num_inspected_values[0] * monkey_index_to_num_inspected_values[1];

    println!("Answer: {}", answer);

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}
