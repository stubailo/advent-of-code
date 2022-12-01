use std::env;
use std::fs;

fn main() {
    // Set file path to a string with "input.txt"
    let file_path = "input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut max_calories_for_elf = 0;
    let mut calories_for_current_elf = 0;
    contents.lines().for_each(|line| {
        // Check if line is empty
        if line.is_empty() {
            if calories_for_current_elf > max_calories_for_elf {
                max_calories_for_elf = calories_for_current_elf;
            }
            calories_for_current_elf = 0;
            return;
        }

        // Parse current line as an integer
        let calories = line
            .parse::<i32>()
            .expect("Should have been able to parse the line as an integer");
        calories_for_current_elf += calories;
    });

    println!("Max calories for elf: {}", max_calories_for_elf);
}
