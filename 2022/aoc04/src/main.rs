use std::fs;

fn main() {
    // Set file path to a string with "input.txt"
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut iter = contents.lines();

    let mut num_contained = 0;

    // Loop through each line in the file, 3 at a time
    loop {
        let line = iter.next();

        // If line is None, break out of the loop
        if line == None {
            break;
        }

        // Get 4 numbers out of the line using a pattern like W-X,Y-Z
        let numbers: Vec<i32> = line
            .unwrap()
            .split(|c| c == '-' || c == ',')
            .map(|str| str.parse::<i32>().expect("could not parse as int"))
            .collect();

        // Decide whether the ranges overlap
        // [0, 1], [2, 3]

        if numbers[0] >= numbers[2] && numbers[0] <= numbers[3]
            || numbers[1] >= numbers[2] && numbers[1] <= numbers[3]
            || numbers[2] >= numbers[0] && numbers[2] <= numbers[1]
            || numbers[3] >= numbers[0] && numbers[3] <= numbers[1]
        {
            num_contained += 1;
        }
    }

    println!("Number of contained: {}", num_contained);
}
