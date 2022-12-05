use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut result = 0;

    // Loop through each line in the file, 3 at a time
    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        // print line
        println!("{}", line);

        // add 1 to result
        result += 1;
    }

    println!("Result: {}", result);
}
