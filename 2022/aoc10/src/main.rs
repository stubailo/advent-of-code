use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut value_of_x_at_cycle: Vec<i32> = Vec::new();
    let mut current_signal_strength: i32 = 1;
    let mut _current_cycle: i32 = 0;

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        // split line on whitespace
        let mut iter2 = line.split_whitespace();

        // get the first word
        let word = iter2.next().unwrap();

        match word {
            "addx" => {
                value_of_x_at_cycle.push(current_signal_strength);
                _current_cycle += 1;

                value_of_x_at_cycle.push(current_signal_strength);
                _current_cycle += 1;

                let number = iter2.next().unwrap().parse::<i32>().unwrap();

                current_signal_strength += number;
            }
            "noop" => {
                value_of_x_at_cycle.push(current_signal_strength);
                _current_cycle += 1;
            }
            _ => {
                panic!("Unknown instruction");
            }
        }
    }

    for i in 0..value_of_x_at_cycle.len() as i32 {
        let x = value_of_x_at_cycle[i as usize];

        let horizontal_position = i % 40;

        if horizontal_position == x - 1 || horizontal_position == x + 1 || horizontal_position == x
        {
            print!("#");
        } else {
            print!(".");
        }

        if (i + 1) % 40 == 0 {
            println!("");
        }
    }

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}
