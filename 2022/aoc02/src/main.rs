use std::fs;

// Enum of rock, paper, scissors
enum RPS {
    Rock,
    Paper,
    Scissors,
}

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
        let mut line_score: i32 = 0;

        let shape_to_play: RPS;

        // Use a match statement to determine the score
        match (words[0], words[1]) {
            ("A", "X") => shape_to_play = RPS::Scissors,
            ("A", "Y") => shape_to_play = RPS::Rock,
            ("A", "Z") => shape_to_play = RPS::Paper,
            ("B", "X") => shape_to_play = RPS::Rock,
            ("B", "Y") => shape_to_play = RPS::Paper,
            ("B", "Z") => shape_to_play = RPS::Scissors,
            ("C", "X") => shape_to_play = RPS::Paper,
            ("C", "Y") => shape_to_play = RPS::Scissors,
            ("C", "Z") => shape_to_play = RPS::Rock,
            _ => panic!("Invalid input"),
        }

        // Add the score for win/draw/lose
        line_score += match words[1] {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("Invalid input"),
        };

        // Add the score for the move -- 1 for X, 2 for Y, 3 for Z
        line_score += match shape_to_play {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };

        // Add the score for the line to the total score
        score += line_score;
    });

    println!("Total score: {}", score);
}
