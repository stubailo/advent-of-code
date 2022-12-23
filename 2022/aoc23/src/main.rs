#![allow(dead_code, unused)]

use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn vector(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    fn locations_to_check(&self) -> [(i32, i32); 3] {
        match self {
            Direction::North => [(-1, -1), (-1, 0), (-1, 1)],
            Direction::South => [(1, -1), (1, 0), (1, 1)],
            Direction::East => [(-1, 1), (0, 1), (1, 1)],
            Direction::West => [(-1, -1), (0, -1), (1, -1)],
        }
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut elves: HashSet<(i32, i32)> = HashSet::new();
    let mut row = 0;

    let mut order_to_consider_directions: VecDeque<Direction> = VecDeque::new();
    order_to_consider_directions.push_back(Direction::North);
    order_to_consider_directions.push_back(Direction::South);
    order_to_consider_directions.push_back(Direction::West);
    order_to_consider_directions.push_back(Direction::East);

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((row, i as i32));
            }
        }
        row += 1;
    }

    // print_elves(&elves);

    let mut step_where_no_elves_moved = 0;

    for step in 0..1000000 {
        if step % 1000 == 0 {
            println!("Step {}", step + 1);
        }

        let mut proposal_count: HashMap<(i32, i32), i32> = HashMap::new();
        // key is elf location, value is proposed move
        let mut proposals_by_elf: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

        for (row, col) in elves.iter() {
            // if all locations around the elf are empty, do nothing
            let mut any_occupied = false;
            for loc in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let row_c = row + loc.0;
                let col_c = col + loc.1;

                if elves.contains(&(row_c, col_c)) {
                    any_occupied = true;
                }
            }

            if !any_occupied {
                continue;
            }

            for proposed_dir in &order_to_consider_directions {
                let mut good_direction = true;

                for (row_c, col_c) in proposed_dir.locations_to_check() {
                    let row_c = row + row_c;
                    let col_c = col + col_c;

                    if elves.contains(&(row_c, col_c)) {
                        good_direction = false;
                    }
                }

                if good_direction {
                    let (row_c, col_c) = proposed_dir.vector();
                    let proposal = (row + row_c, col + col_c);

                    let count = proposal_count.get(&proposal);
                    proposal_count.insert(
                        proposal,
                        match count {
                            Some(c) => *c + 1,
                            None => 1,
                        },
                    );

                    proposals_by_elf.insert((*row, *col), proposal);
                    break;
                }
            }
        }

        let mut any_elves_moved = false;
        for (elf_loc, proposal) in proposals_by_elf {
            if let Some(v) = proposal_count.get(&proposal).or(Some(&0)) {
                if *v <= 1 {
                    elves.remove(&elf_loc);
                    elves.insert(proposal);
                    any_elves_moved = true;
                }
            }
        }

        // Rotate the directions
        let dir = order_to_consider_directions.pop_front().unwrap();
        order_to_consider_directions.push_back(dir);

        // println!("Step {}", step + 1);
        // print_elves(&elves);

        if !any_elves_moved {
            step_where_no_elves_moved = step + 1;
            break;
        }
    }

    println!("Result is {}", step_where_no_elves_moved);

    println!("Time elapsed is: {:?}", start.elapsed());
}

fn print_elves(elves: &HashSet<(i32, i32)>) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for (x, y) in elves {
        if *x < min_x {
            min_x = *x;
        }

        if *x > max_x {
            max_x = *x;
        }

        if *y < min_y {
            min_y = *y;
        }

        if *y > max_y {
            max_y = *y;
        }
    }

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

fn count_empty_locations(elves: &HashSet<(i32, i32)>) -> i32 {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for (x, y) in elves {
        if *x < min_x {
            min_x = *x;
        }

        if *x > max_x {
            max_x = *x;
        }

        if *y < min_y {
            min_y = *y;
        }

        if *y > max_y {
            max_y = *y;
        }
    }

    let mut empties = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !elves.contains(&(x, y)) {
                empties += 1;
            }
        }
    }

    return empties;
}
