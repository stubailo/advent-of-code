use std::fs;

fn main() {
    // Set file path to a string with "input.txt"
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut total_priority: u32 = 0;

    let mut iter = contents.lines();

    // Loop through each line in the file, 3 at a time
    loop {
        let first = iter.next();
        let second = iter.next();
        let third = iter.next();

        // If we've reached the end of the file, break out of the loop
        if first.is_none() || second.is_none() || third.is_none() {
            break;
        }

        let first_char_set: std::collections::HashSet<_> =
            first.expect("string is missing").chars().collect();
        let second_char_set: std::collections::HashSet<_> =
            second.expect("string is missing").chars().collect();
        let third_char_set: std::collections::HashSet<_> =
            third.expect("string is missing").chars().collect();

        // Find the intersection of the three sets
        let intersection1: std::collections::HashSet<_> =
            first_char_set.intersection(&second_char_set).collect();
        let intersection2: std::collections::HashSet<_> =
            second_char_set.intersection(&third_char_set).collect();

        let intersection: Vec<_> = intersection1.intersection(&intersection2).collect();

        // Get single character from intersection
        let character = intersection[0];

        // Convert char to char code
        let character_code = **character as u8;

        let priority: u8;
        // Minus 96 for lowercase, minus 64 for uppercase
        if character_code >= 97 {
            priority = character_code - 96;
        } else {
            priority = character_code - 64 + 26;
        }

        total_priority += priority as u32;
    }

    println!("Total priority: {}", total_priority);
}
