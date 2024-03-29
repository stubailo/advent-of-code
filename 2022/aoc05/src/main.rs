use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut iter = contents.lines();

    // Create a vector of vectors to hold a stack of characters
    let mut stacks: Vec<Vec<char>> = Vec::new();

    // Initialize with 9 vectors
    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    let mut importing_initial_stacks = true;

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        if importing_initial_stacks {
            for i in 0..9 {
                let char = line.chars().nth(i * 4 + 1);

                if char.is_none() {
                    continue;
                }

                let char = char.unwrap();

                if char == ' ' {
                    continue;
                }

                // skip if char is a number
                if char.is_numeric() {
                    continue;
                }

                stacks[i].push(char);
            }

            if line.is_empty() {
                importing_initial_stacks = false;
                // reverse every vector in stacks
                for stack in stacks.iter_mut() {
                    stack.reverse();
                }

                continue;
            }
            continue;
        }

        // parse 3 numbers out of the line like "move 5 from 4 to 5"
        let mut iter = line.split_whitespace();
        iter.next(); // skip "move"
        let num = iter.next().unwrap().parse::<usize>().unwrap();
        iter.next(); // skip "from"
        let from = iter.next().unwrap().parse::<usize>().unwrap();
        iter.next(); // skip "to"
        let to = iter.next().unwrap().parse::<usize>().unwrap();

        // pop num cards from from stack
        let mut cards = Vec::new();
        for _ in 0..num {
            cards.push(stacks[from - 1].pop().unwrap());
        }

        // push cards to to stack
        for card in cards.iter().rev() {
            stacks[to - 1].push(*card);
        }
    }

    // print last item in each stack
    for stack in stacks.iter() {
        let last = stack.last().unwrap(); // we want to error if a stack is empty
        print!("{}", last);
    }

    println!("");

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}
