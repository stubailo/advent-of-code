use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut max_score = 0;

    let mut trees: Vec<Vec<u32>> = Vec::new();

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        let mut tree_row: Vec<u32> = Vec::new();

        for c in line.chars() {
            tree_row.push(c.to_digit(10).unwrap());
        }

        trees.push(tree_row);
    }

    // do all rows; then do all columns
    for row_index in 0..trees.len() {
        let row = &trees[row_index];

        for col_index in 0..row.len() {
            let tree_height = row[col_index];

            let score = calc_scenic_score(&trees, row_index, col_index, tree_height);

            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Max score is: {}", max_score);

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}

fn check_visible(
    trees: &Vec<Vec<u32>>,
    row_index: usize,
    col_index: usize,
    tree_height: u32,
) -> bool {
    // iterate from 0 to row_index
    return (0..col_index).all(|i| trees[row_index][i] < tree_height)
        || (col_index + 1..trees[row_index].len()).all(|i| trees[row_index][i] < tree_height)
        || (0..row_index).all(|i| trees[i][col_index] < tree_height)
        || (row_index + 1..trees.len()).all(|i| trees[i][col_index] < tree_height);
}

fn calc_scenic_score(
    trees: &Vec<Vec<u32>>,
    row_index: usize,
    col_index: usize,
    tree_height: u32,
) -> u32 {
    let mut visible_up = 0;
    for i in (0..row_index).rev() {
        visible_up += 1;
        if trees[i][col_index] >= tree_height {
            break;
        }
    }

    let mut visible_down = 0;
    for i in (row_index + 1)..trees.len() {
        visible_down += 1;
        if trees[i][col_index] >= tree_height {
            break;
        }
    }

    let mut visible_left = 0;
    for i in (0..col_index).rev() {
        visible_left += 1;
        if trees[row_index][i] >= tree_height {
            break;
        }
    }

    let mut visible_right = 0;
    for i in (col_index + 1)..trees[row_index].len() {
        visible_right += 1;
        if trees[row_index][i] >= tree_height {
            break;
        }
    }

    // print all of them
    // println!(
    //     "{}, {} ({}): up {}, down {}, left {}, right {}",
    //     row_index, col_index, tree_height, visible_up, visible_down, visible_left, visible_right
    // );

    return visible_up * visible_down * visible_left * visible_right;
}
