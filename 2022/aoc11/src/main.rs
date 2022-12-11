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

        let _line = line.unwrap();
    }

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}
