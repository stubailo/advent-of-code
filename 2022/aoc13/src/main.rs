use core::str::Chars;
use std::fs;
use std::time::Instant;

struct VecOrNum {
    is_vec: bool,
    vec: Vec<VecOrNum>,
    num: i32,
}

fn main() {
    let start = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut on_line_1 = true;
    let mut line1 = "";
    let mut line2: &str;
    let mut _pair_index = 0;

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        if line.is_empty() {
            on_line_1 = true;
            continue;
        }

        if on_line_1 {
            line1 = line;
            on_line_1 = false;
        } else {
            line2 = line;
            on_line_1 = true;

            // time to process
            _pair_index += 1;

            let mut line1_iter = line1.chars();
            let mut line2_iter = line2.chars();

            let line1_parsed = parse(&mut line1_iter);
            let line2_parsed = parse(&mut line2_iter);

            println!("PARSED");
            print(&line1_parsed);
            print(&line2_parsed);
        }
    }

    println!("Time elapsed is: {:?}", start.elapsed());
}

fn parse(chars: &mut Chars) -> VecOrNum {
    let next_char = chars.next().unwrap();

    if next_char.is_digit(10) {
        let mut number = next_char.to_digit(10).unwrap() as i32;

        loop {
            let next_char = chars.next().unwrap();
            if next_char.is_digit(10) {
                number = number * 10 + next_char.to_digit(10).unwrap() as i32;
            } else {
                break;
            }
        }

        return VecOrNum {
            is_vec: false,
            vec: Vec::new(),
            num: number,
        };
    } else if next_char == '[' {
        let mut vec = Vec::new();

        loop {
            let next_char = chars.next().unwrap();
            if next_char == ']' {
                break;
            } else if next_char == ',' {
                continue;
            } else {
                let parsed = parse(chars);
                vec.push(parsed);
            }
        }

        return VecOrNum {
            is_vec: true,
            vec,
            num: 0,
        };
    } else if next_char == ',' || next_char == ']' {
        return parse(chars);
    } else {
        panic!("Unexpected character {}", next_char);
    }
}

fn print(parsed: &VecOrNum) {
    if parsed.is_vec {
        print!("[");
        for i in 0..parsed.vec.len() {
            print!("{}, ", parsed.vec[i].num);
        }
        print!("]");
    } else {
        print!("{}", parsed.num);
    }
}
