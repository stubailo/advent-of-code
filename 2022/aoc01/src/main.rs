use std::fs;

fn main() {
    // Set file path to a string with "input.txt"
    let file_path = "input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Initialize an array for the top 3 elves
    let mut top_elves = [0; 3];

    let mut calories_for_current_elf = 0;
    contents.lines().for_each(|line| {
        // Check if line is empty
        if line.is_empty() {
            if calories_for_current_elf > top_elves[0] {
                top_elves[0] = calories_for_current_elf;
                top_elves.sort();
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

    // Add up items in top_elves
    let total_calories = top_elves.iter().sum::<i32>();

    println!("Sum for top 3 elves: {}", total_calories);
}
