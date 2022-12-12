use std::collections::HashSet;
use std::fs;
use std::time::Instant;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let start = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut elevations: Vec<Vec<i32>> = Vec::new();

    let mut ending_point: Point = Point { x: 0, y: 0 };

    let mut row_num = 0;

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        let mut col_num = 0;

        // add line to elevations, translate a-z into numbers 0-25
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            if c == 'S' {
                row.push(0);
            } else if c == 'E' {
                ending_point = Point {
                    x: row_num,
                    y: col_num,
                };
                row.push(25);
            } else {
                row.push(c as i32 - 'a' as i32);
            }
            col_num += 1;
        }
        elevations.push(row);

        row_num += 1;
    }

    // Make a vector of all Points in elevations with elevation 0
    let mut starting_points: Vec<Point> = Vec::new();
    for i in 0..elevations.len() {
        for j in 0..elevations[0].len() {
            if elevations[i][j] == 0 {
                starting_points.push(Point {
                    x: i as i32,
                    y: j as i32,
                });
            }
        }
    }

    let mut min_num_steps = std::i32::MAX;

    for starting_point in starting_points {
        let mut num_steps = 0;
        let mut found = false;

        let mut visited_locations: HashSet<Point> = HashSet::new();
        let mut current_locations: Vec<Point> = Vec::new();

        visited_locations.insert(starting_point);
        current_locations.push(starting_point);

        'outer: loop {
            num_steps += 1;

            // break outer if there are no more current locations
            if current_locations.len() == 0 {
                break 'outer;
            }

            let mut next_locations: Vec<Point> = Vec::new();

            for loc in current_locations {
                let next_locs = vec![
                    Point {
                        x: loc.x + 1,
                        y: loc.y,
                    },
                    Point {
                        x: loc.x - 1,
                        y: loc.y,
                    },
                    Point {
                        x: loc.x,
                        y: loc.y + 1,
                    },
                    Point {
                        x: loc.x,
                        y: loc.y - 1,
                    },
                ];

                for next_loc in next_locs {
                    if visited_locations.contains(&next_loc) {
                        continue;
                    }

                    if next_loc.x >= elevations.len() as i32
                        || next_loc.y >= elevations[0].len() as i32
                        || next_loc.x < 0
                        || next_loc.y < 0
                    {
                        continue;
                    }

                    if elevations[next_loc.x as usize][next_loc.y as usize]
                        > elevations[loc.x as usize][loc.y as usize] + 1
                    {
                        continue;
                    }

                    if next_loc == ending_point {
                        found = true;
                        break 'outer;
                    }

                    visited_locations.insert(next_loc);
                    next_locations.push(next_loc);
                }
            }

            current_locations = next_locations;
        }

        if found {
            if num_steps < min_num_steps {
                min_num_steps = num_steps;
            }
        }
    }

    println!("Num steps: {}", min_num_steps);

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}
