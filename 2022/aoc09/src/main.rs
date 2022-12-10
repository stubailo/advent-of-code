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

    let mut rope_segment_positions: Vec<Point> = Vec::new();
    // add 10 points to the rope segment positions that are all 0, 0
    for _ in 0..10 {
        rope_segment_positions.push(Point { x: 0, y: 0 });
    }

    // create a set of Points that represent all positions the tail has been in
    let mut tail_positions: HashSet<Point> = HashSet::new();

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        // split line on space
        let mut iter = line.split(" ");

        // get first item as a character
        let direction = iter.next().unwrap().chars().next().unwrap();

        // get second item as a number
        let distance = iter.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..distance {
            match direction {
                'U' => rope_segment_positions[0].y += 1,
                'D' => rope_segment_positions[0].y -= 1,
                'L' => rope_segment_positions[0].x -= 1,
                'R' => rope_segment_positions[0].x += 1,
                _ => panic!("Unknown direction"),
            }

            // now determine if the tail needs to move
            for i in 1..rope_segment_positions.len() {
                // if the new head position is within 1 unit of the tail position
                // then we don't need to move the tail
                if (rope_segment_positions[i - 1].x - rope_segment_positions[i].x).abs() <= 1
                    && (rope_segment_positions[i - 1].y - rope_segment_positions[i].y).abs() <= 1
                {
                    continue;
                }

                // if the new head position is exactly 2 units away from the tail position, move it one closer
                if (rope_segment_positions[i - 1].x - rope_segment_positions[i].x).abs() == 2
                    && (rope_segment_positions[i - 1].y - rope_segment_positions[i].y).abs() == 0
                {
                    if rope_segment_positions[i - 1].x > rope_segment_positions[i].x {
                        rope_segment_positions[i].x += 1;
                    } else {
                        rope_segment_positions[i].x -= 1;
                    }

                    continue;
                } else if (rope_segment_positions[i - 1].x - rope_segment_positions[i].x).abs() == 0
                    && (rope_segment_positions[i - 1].y - rope_segment_positions[i].y).abs() == 2
                {
                    if rope_segment_positions[i - 1].y > rope_segment_positions[i].y {
                        rope_segment_positions[i].y += 1;
                    } else {
                        rope_segment_positions[i].y -= 1;
                    }
                    continue;
                }

                // if the new head position is 2 steps along one axis and 1 step along the other axis,
                // move it 1 step along both axes
                if (rope_segment_positions[i - 1].x - rope_segment_positions[i].x).abs() == 2
                    && (rope_segment_positions[i - 1].y - rope_segment_positions[i].y).abs() >= 1
                {
                    if rope_segment_positions[i - 1].x > rope_segment_positions[i].x {
                        rope_segment_positions[i].x += 1;
                    } else {
                        rope_segment_positions[i].x -= 1;
                    }

                    if rope_segment_positions[i - 1].y > rope_segment_positions[i].y {
                        rope_segment_positions[i].y += 1;
                    } else {
                        rope_segment_positions[i].y -= 1;
                    }

                    continue;
                } else if (rope_segment_positions[i - 1].x - rope_segment_positions[i].x).abs() >= 1
                    && (rope_segment_positions[i - 1].y - rope_segment_positions[i].y).abs() == 2
                {
                    if rope_segment_positions[i - 1].x > rope_segment_positions[i].x {
                        rope_segment_positions[i].x += 1;
                    } else {
                        rope_segment_positions[i].x -= 1;
                    }

                    if rope_segment_positions[i - 1].y > rope_segment_positions[i].y {
                        rope_segment_positions[i].y += 1;
                    } else {
                        rope_segment_positions[i].y -= 1;
                    }

                    continue;
                }

                // if we get here, panic
                panic!("Unknown case");
            }

            // add the tail position to the set of tail positions
            tail_positions.insert(rope_segment_positions[rope_segment_positions.len() - 1]);
        }
    }

    // print the number of different tail positions
    println!("Number of tail positions: {}", tail_positions.len());

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}
