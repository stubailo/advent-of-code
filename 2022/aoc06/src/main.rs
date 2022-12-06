use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut iter = contents.lines();

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        let mut i = 0;

        let mut last_fourteen: VecDeque<char> = VecDeque::new();

        // iterate over line one character at a time
        for c in line.chars() {
            i += 1;

            if last_fourteen.len() < 14 {
                last_fourteen.push_back(c);
                continue;
            }

            last_fourteen.pop_front();
            last_fourteen.push_back(c);

            // make a set out of the items in last_fourteen
            let mut set: HashSet<char> = HashSet::new();
            for c2 in last_fourteen.iter() {
                set.insert(*c2);
            }

            // if the set has 4 items, then we have a winner
            if set.len() == 14 {
                println!("num characters: {}", i);
                println!("chars: {}", last_fourteen.iter().collect::<String>());
                break;
            }
        }
    }

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}
