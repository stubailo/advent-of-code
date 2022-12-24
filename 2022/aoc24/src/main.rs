#![allow(dead_code, unused)]

use core::num;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

#[derive(Debug, Copy, Clone)]
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

    fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::North),
            'v' => Some(Direction::South),
            '>' => Some(Direction::East),
            '<' => Some(Direction::West),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        }
    }
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut blizzards: HashMap<(i32, i32), Vec<Direction>> = HashMap::new();

    // skip first line
    let first_line = iter.next();
    let num_cols = (first_line.unwrap().len() as i32) - 2;

    let mut row = 0;

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        for (i, c) in line.chars().enumerate() {
            let blizz = Direction::from_char(c);

            if blizz.is_some() {
                let col = i as i32 - 1;
                let blizz = blizz.unwrap();

                let entry = blizzards.entry((row, col)).or_insert(Vec::new());
                entry.push(blizz);
            }
        }

        row += 1;
    }

    let num_rows = row - 1;

    //println!("Initial blizzards: ");
    //print_blizzards(&blizzards, num_rows, num_cols);

    let mut next_step_list: HashSet<(i32, i32)> = HashSet::new();

    next_step_list.insert((-1, 0));

    let mut num_steps_to_find = 0;

    // 0 is going to destination, 1 is coming back to start, 2 is going to destination
    let mut phase = 0;
    let mut goals = [(num_rows, num_cols - 1), (-1, 0), (num_rows, num_cols - 1)];

    'steps: for step in 1..=1000 {
        blizzards = simulate_blizzard_step(&blizzards, num_rows, num_cols);

        // println!("Step: {}", step);
        // print_blizzards(&blizzards, num_rows, num_cols);

        let mut new_next_step_list: HashSet<(i32, i32)> = HashSet::new();

        'locations: for loc in &next_step_list {
            for direction in [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ] {
                let (dr, dc) = direction.vector();
                let new_r = loc.0 + dr;
                let new_c = loc.1 + dc;

                if new_r == goals[phase].0 && new_c == goals[phase].1 {
                    println!("Found phase {} goal at step: {}", phase, step);

                    if phase == 2 {
                        num_steps_to_find = step;
                        break 'steps;
                    } else {
                        phase += 1;
                        new_next_step_list = HashSet::new();
                        new_next_step_list.insert((new_r, new_c));
                        break 'locations;
                    }
                }

                if 0 <= new_r
                    && new_r < num_rows
                    && 0 <= new_c
                    && new_c < num_cols
                    && !blizzards.contains_key(&(new_r, new_c))
                {
                    new_next_step_list.insert((new_r, new_c));
                } else if (new_r, new_c) == (-1, 0) || (new_r, new_c) == (num_rows, num_cols - 1) {
                    new_next_step_list.insert((new_r, new_c));
                }
            }

            if !blizzards.contains_key(loc) {
                new_next_step_list.insert(*loc);
            }
        }

        next_step_list = new_next_step_list;
    }

    println!("Num steps to find: {}", num_steps_to_find);

    println!("Time elapsed is: {:?}", start.elapsed());
}

fn simulate_blizzard_step(
    blizzards: &HashMap<(i32, i32), Vec<Direction>>,
    num_rows: i32,
    num_cols: i32,
) -> HashMap<(i32, i32), Vec<Direction>> {
    let mut new_blizzards: HashMap<(i32, i32), Vec<Direction>> = HashMap::new();

    for ((row, col), blizz_vec) in blizzards {
        for blizz in blizz_vec {
            let (dr, dc) = blizz.vector();
            let new_row = (row + dr + num_rows) % num_rows;
            let new_col = (col + dc + num_cols) % num_cols;

            let entry = new_blizzards
                .entry((new_row, new_col))
                .or_insert(Vec::new());
            entry.push(*blizz);
        }
    }

    new_blizzards
}

fn print_blizzards(blizzards: &HashMap<(i32, i32), Vec<Direction>>, num_rows: i32, num_cols: i32) {
    for row in 0..num_rows {
        for col in 0..num_cols {
            if blizzards.contains_key(&(row, col)) {
                let blizzard_vec = blizzards.get(&(row, col)).unwrap();

                if blizzard_vec.len() == 1 {
                    print!("{}", blizzard_vec[0].to_char());
                } else {
                    print!("{}", blizzard_vec.len());
                }
            } else {
                print!(".");
            }
        }

        println!();
    }
}
