use std::fs;
use std::time::Instant;

struct Monkey {
    items: Vec<u32>,
    operation: String,
    test_divisible_by: u32,
    next_monkey_if_true: u32,
    next_monkey_if_false: u32,
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

            // print line
            println!("{}", line);

            if line.is_empty() {
                monkeys.push(current_monkey);
                continue 'outer;
            } else if line.starts_with("Monkey") {
                // do nothing
            } else if line.trim().starts_with("Starting") {
                line.split(": ").nth(1).unwrap().split(", ").for_each(|x| {
                    current_monkey.items.push(x.parse::<u32>().unwrap());
                });
                // print current_monkey.items
                println!("Items: {:?}", current_monkey.items)
            } else if line.trim().starts_with("Operation") {
                current_monkey.operation = line.split("= ").last().unwrap().to_string();
            } else if line.trim().starts_with("Test") {
                current_monkey.test_divisible_by =
                    line.split(" ").last().unwrap().parse::<u32>().unwrap();
            } else if line.trim().starts_with("If true") {
                current_monkey.next_monkey_if_true =
                    line.split(" ").last().unwrap().parse::<u32>().unwrap();
            } else if line.trim().starts_with("If false") {
                current_monkey.next_monkey_if_false =
                    line.split(" ").last().unwrap().parse::<u32>().unwrap();
            }
        }
    }

    // Print out the vector of monkeys
    for monkey in &monkeys {
        println!("Monkey:");
        println!("  Starting items: {:?}", monkey.items);
        println!("  Operation: {}", monkey.operation);
        println!("  Test: divisible by {}", monkey.test_divisible_by);
        println!(
            "    If true: throw to monkey {}",
            monkey.next_monkey_if_true
        );
        println!(
            "    If false: throw to monkey {}",
            monkey.next_monkey_if_false
        );
    }

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}
