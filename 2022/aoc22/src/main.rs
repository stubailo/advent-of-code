#![allow(dead_code, unused)]

use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
enum Step {
    TurnLeft,
    TurnRight,
    Forward(i32),
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut board: HashMap<(i32, i32), char> = HashMap::new();

    let mut num_points = 0;
    let mut y = 0;
    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        if line.is_empty() {
            break;
        }

        for (i, c) in line.chars().enumerate() {
            if c != '#' && c != '.' {
                continue;
            }

            board.insert((y, i as i32), c);
            num_points += 1;
        }

        y += 1;
    }

    let edge_length = ((num_points / 6) as f32).sqrt();
    // print
    println!("Edge length: {}", edge_length);

    let line = iter.next().unwrap();

    let mut directions: Vec<Step> = Vec::new();

    let mut iter = line.chars().peekable();
    loop {
        let char = iter.next();

        if char.is_none() {
            break;
        }

        let char = char.unwrap();

        if char == 'L' {
            directions.push(Step::TurnLeft);
        } else if char == 'R' {
            directions.push(Step::TurnRight);
        } else if char.is_digit(10) {
            let mut num = char.to_digit(10).unwrap() as i32;
            loop {
                let next_char = iter.peek();

                if next_char.is_none() || !next_char.unwrap().is_digit(10) {
                    directions.push(Step::Forward(num));
                    break;
                }

                num = num * 10 + iter.next().unwrap().to_digit(10).unwrap() as i32;
            }
        }
    }

    // done parsing

    // find minimum col in row 0
    let mut min_col = 100000;
    for (row, col) in board.keys() {
        if *row == 0 && *col < min_col {
            min_col = *col;
        }
    }

    let mut location: (i32, i32) = (0, min_col);
    let mut direction: (i32, i32) = (0, 1);

    // println!("Location: {:?}", location);

    for step in directions {
        match step {
            Step::TurnLeft => {
                if direction == (0, 1) {
                    direction = (-1, 0);
                } else if direction == (-1, 0) {
                    direction = (0, -1);
                } else if direction == (0, -1) {
                    direction = (1, 0);
                } else if direction == (1, 0) {
                    direction = (0, 1);
                }
            }
            Step::TurnRight => {
                if direction == (0, 1) {
                    direction = (1, 0);
                } else if direction == (1, 0) {
                    direction = (0, -1);
                } else if direction == (0, -1) {
                    direction = (-1, 0);
                } else if direction == (-1, 0) {
                    direction = (0, 1);
                }
            }
            Step::Forward(distance) => {
                for _ in 0..distance {
                    let mut next_location = (location.0 + direction.0, location.1 + direction.1);

                    if board.get(&next_location).is_none() {
                        get_next_location_and_rotation_if_you_exit_the_cube(location, direction);

                        // get opposite direction
                        let opposite_direction = (-direction.0, -direction.1);

                        loop {
                            next_location = (
                                next_location.0 + opposite_direction.0,
                                next_location.1 + opposite_direction.1,
                            );

                            if board.get(&next_location).is_none() {
                                next_location = (
                                    next_location.0 - opposite_direction.0,
                                    next_location.1 - opposite_direction.1,
                                );
                                break;
                            }
                        }
                    }

                    if board.get(&next_location).unwrap() == &'#' {
                        break;
                    }

                    location = next_location;

                    // print location
                    // println!("Location: {:?}", location);
                }
            }
        }
    }

    let final_row = location.0 + 1;
    let final_col = location.1 + 1;
    let final_direction_score = match direction {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => panic!("Invalid direction"),
    };

    let result = 1000 * final_row + 4 * final_col + final_direction_score;

    println!("Result: {}", result);

    println!("Time elapsed is: {:?}", start.elapsed());
}

fn get_next_location_and_rotation_if_you_exit_the_cube(
    location: (i32, i32),
    direction: (i32, i32),
) -> ((i32, i32), (i32, i32)) {
    // first, which segment are we in?

    // . A B
    // . C
    // D E
    // F

    let area: char;

    if location.0 < 50 {
        if 50 <= location.1 && location.1 < 100 {
            area = 'A';
        } else if 100 <= location.1 && location.1 < 150 {
            area = 'B';
        } else {
            panic!("Location not in cube: {:?}", location);
        }
    } else if 50 <= location.0 && location.0 < 100 {
        if 50 <= location.1 && location.1 < 100 {
            area = 'C';
        } else {
            panic!("Location not in cube: {:?}", location);
        }
    } else if 100 <= location.0 && location.0 < 150 {
        if 0 <= location.1 && location.1 < 50 {
            area = 'D';
        } else if 50 <= location.1 && location.1 < 100 {
            area = 'E';
        } else {
            panic!("Location not in cube: {:?}", location);
        }
    } else if 150 <= location.0 && location.0 < 200 {
        if 0 <= location.1 && location.1 < 50 {
            area = 'F';
        } else {
            panic!("Location not in cube: {:?}", location);
        }
    } else {
        panic!("Location not in cube: {:?}", location);
    }

    println!("Area: {}", area);

    // . A B
    // . C
    // D E
    // F

    // Match all combinations of area and direction
    match (area, direction) {
        // ('A', (0, 1)) => return ((50, 50), (1, 0)),
        // ('A', (1, 0)) => return ((50, 50), (0, -1)),
        ('A', (0, -1)) => return ((50, 50), (-1, 0)),
        ('A', (-1, 0)) => return ((50, 50), (0, 1)),
        ('B', (0, 1)) => return ((50, 100), (1, 0)),
        ('B', (1, 0)) => return ((50, 100), (0, 1)),
        // ('B', (0, -1)) => return ((50, 100), (-1, 0)),
        ('B', (-1, 0)) => return ((50, 100), (0, -1)),
        ('C', (0, 1)) => return ((100, 50), (1, 0)),
        // ('C', (1, 0)) => return ((100, 50), (0, -1)),
        ('C', (0, -1)) => return ((100, 50), (-1, 0)),
        // ('C', (-1, 0)) => return ((100, 50), (0, 1)),
        // ('D', (0, 1)) => return ((100, 0), (0, -1)),
        // ('D', (1, 0)) => return ((100, 0), (1, 0)),
        ('D', (0, -1)) => return ((100, 0), (0, 1)),
        ('D', (-1, 0)) => return ((100, 0), (-1, 0)),
        ('E', (0, 1)) => return ((100, 50), (0, -1)),
        ('E', (1, 0)) => return ((100, 50), (1, 0)),
        // ('E', (0, -1)) => return ((100, 50), (0, 1)),
        // ('E', (-1, 0)) => return ((100, 50), (-1, 0)),
        ('F', (0, 1)) => return ((100, 50), (0, -1)),
        ('F', (1, 0)) => return ((100, 50), (1, 0)),
        ('F', (0, -1)) => return ((100, 50), (0, 1)),
        // ('F', (-1, 0)) => return ((100, 50), (-1, 0)),
        _ => panic!(
            "Invalid area and direction combination: {} {:?}",
            area, direction
        ),
    }

    return ((0, 0), (0, 0));
}
