use std::fs;

fn main() {
    // Set file path to a string with "input.txt"
    let file_path = "input.txt";

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut score = 0;

    contents.lines().for_each(|line| {
        // Skip empty lines
        if line.is_empty() {
            return;
        }

        // split line into words
        let words: Vec<&str> = line.split_whitespace().collect();

        // The mappings for the first character are:
        // A: rock
        // B: paper
        // C: scissors
        // For the second:
        // X: rock
        // Y: paper
        // Z: scissors

        // If we win, get 6 points, draw is 3 point, lose is 0 points
        let mut line_score;

        // Use a match statement to determine the score
        match (words[0], words[1]) {
            ("A", "X") => line_score = 3,
            ("A", "Y") => line_score = 6,
            ("A", "Z") => line_score = 0,
            ("B", "X") => line_score = 0,
            ("B", "Y") => line_score = 3,
            ("B", "Z") => line_score = 6,
            ("C", "X") => line_score = 6,
            ("C", "Y") => line_score = 0,
            ("C", "Z") => line_score = 3,
            _ => line_score = 0,
        }

        // Add the score for the move -- 1 for X, 2 for Y, 3 for Z
        line_score += match words[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };

        // Add the score for the line to the total score
        score += line_score;
    });

    println!("Total score: {}", score);
}
