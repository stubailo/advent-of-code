use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut num_visible = 0;

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

            if check_visible(&trees, row_index, col_index, tree_height) {
                num_visible += 1;
            }
        }
    }

    println!("Num visible is: {}", num_visible);

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
