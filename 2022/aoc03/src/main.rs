use std::fs;

fn main() {
    // Set file path to a string with "input.txt"
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut total_priority: u32 = 0;

    contents.lines().for_each(|line| {
        let first_half = &line[..line.len() / 2];
        let second_half = &line[line.len() / 2..];

        let first_half_set: std::collections::HashSet<_> = first_half.chars().collect();
        let second_half_set: std::collections::HashSet<_> = second_half.chars().collect();

        let intersection: Vec<_> = first_half_set.intersection(&second_half_set).collect();

        // Get single character from intersection
        let character = intersection[0];

        // Convert char to char code
        let character_code = *character as u8;

        let priority: u8;
        // Minus 96 for lowercase, minus 64 for uppercase
        if character_code >= 97 {
            priority = character_code - 96;
        } else {
            priority = character_code - 64 + 26;
        }

        total_priority += priority as u32;
    });

    println!("Total priority: {}", total_priority);
}
