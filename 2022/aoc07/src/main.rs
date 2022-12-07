use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut iter = contents.lines();

    // path to file size
    let mut paths: HashMap<String, i32> = HashMap::new();

    // directories
    let mut dirs: HashSet<String> = HashSet::new();

    let mut current_dir = "/".to_owned();

    dirs.insert(current_dir.clone());

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        if line.starts_with("$") {
            let new_current_dir: String;

            // check if line starts with $
            if line == "$ cd /" {
                new_current_dir = "/".to_owned();
            } else if line == "$ cd .." {
                let last_slash = current_dir.rfind("/").unwrap();
                new_current_dir = current_dir.chars().collect::<Vec<char>>().as_slice()
                    [..last_slash]
                    .iter()
                    .collect();
            } else if line.starts_with("$ cd ") {
                let dir = &line[5..];
                new_current_dir = format!("{}{}/", current_dir, dir);
                dirs.insert(new_current_dir.clone());
            } else {
                new_current_dir = current_dir.clone();
            }

            current_dir = new_current_dir;

            continue;
        }

        // if line doesn't start with $, it's a file or directory
        // we don't need to consider directories
        let line_parts = line.split(" ");

        // check if first part is a number
        let first_part = line_parts.clone().next().unwrap();
        if first_part.parse::<i32>().is_ok() {
            let file_size = first_part.parse::<i32>().unwrap();
            let file_name = line_parts.clone().last().unwrap();

            let path = format!("{}{}", current_dir, file_name).clone().to_owned();

            paths.insert(path.as_str().to_owned(), file_size);
        }
    }

    // Make a hashmap of dir to total size
    let mut dir_to_size: HashMap<String, i32> = HashMap::new();

    for dir in dirs {
        // Add up the sizes of all files whose names start with dir
        let mut total_size = 0;

        for (path, size) in paths.iter() {
            if path.starts_with(&dir) {
                total_size += size;
            }
        }

        dir_to_size.insert(dir, total_size);
    }

    // Find size of dir /
    let free_space = 70000000 - dir_to_size.get("/").unwrap();

    let space_needed = 30000000 - free_space;

    // Find the smallest dir that is at least of size space_needed
    let mut smallest_dir_size = dir_to_size.get("/").unwrap();

    for (_, size) in dir_to_size.iter() {
        if size >= &space_needed && size < &smallest_dir_size {
            smallest_dir_size = size;
        }
    }

    println!("Smallest dir size is: {}", smallest_dir_size);

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}
