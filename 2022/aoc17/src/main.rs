use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

// ####

// .#.
// ###
// .#.

// ..#
// ..#
// ###

// #
// #
// #
// #

// ##
// ##

const falling_rock_char: char = '@';
const stopped_rock_char: char = '#';
const MAX_WIDTH: i32 = 7;
const DEBUG: bool = false;

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut air_puffs: Vec<char> = Vec::new();
    let mut cur_height = 0;

    // row 0 is the bottom row
    // (row, x), char
    let mut stopped_rocks: HashMap<(u64, i32), char> = HashMap::new();

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        air_puffs = line.chars().collect();
    }

    let mut air_puff_index = 0;

    // print number of air puffs
    println!("Number of air puffs: {}", air_puffs.len());

    // hashset should keep track of: which puff index we are on, which rock index we are on, and
    // which x position it stops at
    let mut cycle_detection_set: HashSet<(usize, usize, i32)> = HashSet::new();

    // cycle length is always 1750, but we need the difference to be the same
    let mut last_height = 0;

    let mut detected_cycle_at: usize = 0;

    for rock_index in 0..1000000000000 {
        let rock_type = rock_index % 5;
        if DEBUG {
            println!("New rock: {}", rock_type);
        }

        let mut falling_rocks: HashMap<(u64, i32), char> = HashMap::new();

        draw_falling_rock(rock_type, cur_height + 3, &mut falling_rocks);

        if DEBUG {
            println!("Initial board state:");
            print_board(&stopped_rocks, &falling_rocks, cur_height + 10, 0);
        }

        // now, simulate the rock getting puffed and also falling
        loop {
            // first, puff
            let air_puff = air_puffs[air_puff_index];
            air_puff_index = (air_puff_index + 1) % air_puffs.len();

            if DEBUG {
                println!("Air puff: {}", air_puff);
            }

            let direction: i32 = match air_puff {
                '>' => 1,
                '<' => -1,
                _ => {
                    panic!("Unknown air puff: {}", air_puff);
                }
            };

            // move
            let mut puffed_falling_rocks: HashMap<(u64, i32), char> = HashMap::new();

            let mut would_collide = false;
            for ((row, x), val) in falling_rocks.iter() {
                if *val == falling_rock_char {
                    if x + direction < 0
                        || x + direction >= MAX_WIDTH
                        || stopped_rocks.contains_key(&(*row, x + direction))
                    {
                        would_collide = true;
                        break;
                    }
                    puffed_falling_rocks.insert((*row, x + direction), *val);
                }
            }

            if !would_collide {
                falling_rocks = puffed_falling_rocks;
            }

            if DEBUG {
                print_board(&stopped_rocks, &falling_rocks, cur_height + 10, 0);
            }

            // now, fall
            let mut fallen_falling_rocks: HashMap<(u64, i32), char> = HashMap::new();

            let mut touched_ground = false;
            for ((row, x), val) in falling_rocks.iter() {
                if *val == falling_rock_char {
                    if *row == 0 || stopped_rocks.contains_key(&(*row - 1, *x)) {
                        // we've touched the ground
                        touched_ground = true;
                        break;
                    }
                    fallen_falling_rocks.insert((*row - 1, *x), *val);
                }
            }

            if touched_ground {
                if DEBUG {
                    println!("Touched ground");
                }
                break;
            } else {
                if DEBUG {
                    println!("Falling");
                }
                falling_rocks = fallen_falling_rocks;
            }

            if DEBUG {
                print_board(&stopped_rocks, &falling_rocks, cur_height + 10, 0);
            }
        }

        // now, move the falling rocks to the stopped rocks

        let mut left_most_x = MAX_WIDTH;
        for ((row, x), val) in falling_rocks.iter() {
            if *val == falling_rock_char {
                stopped_rocks.insert((*row, *x), stopped_rock_char);

                if *x < left_most_x {
                    left_most_x = *x;
                }

                if *row + 1 > cur_height {
                    if DEBUG {
                        println!("Updating cur_height to {}", *row + 1);
                    }
                    cur_height = *row + 1;
                }
            }
        }

        let new_set_item = (air_puff_index, rock_type, left_most_x);

        if cycle_detection_set.contains(&new_set_item) {
            println!("Cycle detected!");
            println!("Current index: {}", rock_index);
            println!("Current height: {}", cur_height);
            println!("Cycle length: {}", cycle_detection_set.len());
            println!("Cycle start: {}", rock_index % cycle_detection_set.len());
            println!("Height diff this cycle: {}", cur_height - last_height);

            if rock_index > 2000 {
                // print top few rows of board
                print_board(
                    &stopped_rocks,
                    &HashMap::new(),
                    cur_height + 3,
                    cur_height - 20,
                );
                detected_cycle_at = rock_index;
            }

            last_height = cur_height;
            cycle_detection_set = HashSet::new();
            // print new set item
            // break;
        }

        if detected_cycle_at > 0 && rock_index == detected_cycle_at + 34 {
            let height_diff = cur_height - last_height;

            println!("Height diff: {}", height_diff);
            break;
        }

        cycle_detection_set.insert(new_set_item);
    }

    if DEBUG {
        println!("Final board state:");
        print_board(&stopped_rocks, &HashMap::new(), cur_height + 10, 0);
    }

    println!("Height: {}", cur_height);

    println!("Time elapsed is: {:?}", start.elapsed());
}

fn draw_falling_rock(
    rock_type: usize,
    desired_lowest_row: u64,
    board: &mut HashMap<(u64, i32), char>,
) {
    // ####
    let patterns: Vec<Vec<Vec<bool>>> = vec![
        vec![vec![true, true, true, true]],
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        vec![
            vec![false, false, true],
            vec![false, false, true],
            vec![true, true, true],
        ],
        vec![vec![true], vec![true], vec![true], vec![true]],
        vec![vec![true, true], vec![true, true]],
    ];

    let mut cur_row = desired_lowest_row;

    for row in patterns[rock_type].iter().rev() {
        for (x, is_filled) in row.iter().enumerate() {
            if *is_filled {
                board.insert((cur_row, (x + 2) as i32), falling_rock_char);
            }
        }

        cur_row += 1;
    }
}

fn print_board(
    stopped_rocks: &HashMap<(u64, i32), char>,
    falling_rocks: &HashMap<(u64, i32), char>,
    max_row: u64,
    min_row: u64,
) {
    for cur_row in (min_row..=max_row).rev() {
        let mut row = String::new();

        for x in 0..MAX_WIDTH {
            let mut val = stopped_rocks.get(&(cur_row, x));

            if val.is_none() {
                val = falling_rocks.get(&(cur_row, x));

                if val.is_none() {
                    row.push('.');
                } else {
                    row.push(*val.unwrap());
                }
            } else {
                row.push(*val.unwrap());
            }
        }

        println!("{} {}", cur_row, row);
    }
}
