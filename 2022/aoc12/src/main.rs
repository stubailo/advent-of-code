use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct PointWithDistance {
    x: i32,
    y: i32,
    steps: i32,
}

fn main() {
    let start = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut elevations: Vec<Vec<i32>> = Vec::new();
    let mut ending_point: PointWithDistance = PointWithDistance {
        x: 0,
        y: 0,
        steps: -1,
    };

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
                ending_point = PointWithDistance {
                    x: row_num,
                    y: col_num,
                    steps: 0,
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

    let num_steps;

    let mut visited_locations: HashSet<(i32, i32)> = HashSet::new();
    let mut current_locations: VecDeque<PointWithDistance> = VecDeque::new();

    // Since we're looking for the zero elevation location that has the shortest path to the end,
    // we can start at the end and work backwards -- the first zero elevation location we find
    // will be the shortest path to the end.
    visited_locations.insert((ending_point.x, ending_point.y));
    current_locations.push_back(ending_point);

    'outer: loop {
        let loc = current_locations.pop_front().unwrap();
        let next_locs = [
            PointWithDistance {
                x: loc.x + 1,
                y: loc.y,
                steps: loc.steps + 1,
            },
            PointWithDistance {
                x: loc.x - 1,
                y: loc.y,
                steps: loc.steps + 1,
            },
            PointWithDistance {
                x: loc.x,
                y: loc.y + 1,
                steps: loc.steps + 1,
            },
            PointWithDistance {
                x: loc.x,
                y: loc.y - 1,
                steps: loc.steps + 1,
            },
        ];

        for next_loc in next_locs {
            if visited_locations.contains(&(next_loc.x, next_loc.y)) {
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
                < elevations[loc.x as usize][loc.y as usize] - 1
            {
                continue;
            }

            if elevations[next_loc.x as usize][next_loc.y as usize] == 0 {
                num_steps = next_loc.steps;
                break 'outer;
            }

            visited_locations.insert((next_loc.x, next_loc.y));
            current_locations.push_back(next_loc);
        }
    }

    println!("Num steps: {}", num_steps);

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}
