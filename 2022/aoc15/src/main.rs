use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        println!("{}", line);
    }

    println!("Time elapsed is: {:?}", start.elapsed());
}
