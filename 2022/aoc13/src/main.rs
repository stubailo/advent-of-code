use core::iter::Peekable;
use core::str::Chars;
use std::fs;
use std::time::Instant;

struct VecOrNum {
    is_vec: bool,
    vec: Vec<VecOrNum>,
    num: i32,
}

#[derive(PartialEq, Debug)]
enum TrueFalseOrNoDecision {
    True,
    False,
    NoDecision,
}

fn main() {
    let start = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut parsed_lines: Vec<VecOrNum> = Vec::new();

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        let mut line_iter = line.chars().peekable();

        let line_parsed = parse(&mut line_iter);

        parsed_lines.push(line_parsed);
    }

    parsed_lines.push(parse(&mut "[[2]]".chars().peekable()));
    parsed_lines.push(parse(&mut "[[6]]".chars().peekable()));

    parsed_lines.sort_by(|a, b| {
        let result = in_correct_order(a, b);

        if result == TrueFalseOrNoDecision::True {
            return std::cmp::Ordering::Less;
        } else if result == TrueFalseOrNoDecision::False {
            return std::cmp::Ordering::Greater;
        } else {
            return std::cmp::Ordering::Equal;
        }
    });

    // go through parsed_lines and find the indices for [[2]] and [[6]]
    let mut index_of_2 = 0;
    let mut index_of_6 = 0;

    for i in 0..parsed_lines.len() {
        let parsed = &parsed_lines[i];

        if parsed.is_vec
            && parsed.vec.len() == 1
            && parsed.vec[0].is_vec
            && parsed.vec[0].vec.len() == 1
            && parsed.vec[0].vec[0].num == 2
        {
            index_of_2 = i + 1;
        } else if parsed.is_vec
            && parsed.vec.len() == 1
            && parsed.vec[0].is_vec
            && parsed.vec[0].vec.len() == 1
            && parsed.vec[0].vec[0].num == 6
        {
            index_of_6 = i + 1;
        }
    }

    // print both indices
    println!("Index of 2 is: {}", index_of_2);
    println!("Index of 6 is: {}", index_of_6);

    // multiply them and print the result
    let result = index_of_2 * index_of_6;

    println!("Result is: {}", result);

    println!("Time elapsed is: {:?}", start.elapsed());
}

fn parse(chars: &mut Peekable<Chars>) -> VecOrNum {
    let next_char = chars.next().unwrap();

    if next_char.is_digit(10) {
        let mut number = next_char.to_digit(10).unwrap() as i32;

        loop {
            let peek = chars.peek().unwrap();

            if peek.is_digit(10) {
                let next_char = chars.next().unwrap();

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
            let peek = chars.peek().unwrap();

            if *peek == ']' {
                chars.next();
                break;
            } else if *peek == ',' {
                chars.next();
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
    } else {
        panic!("Unexpected character {}", next_char);
    }
}

fn print(parsed: &VecOrNum) {
    if parsed.is_vec {
        print!("[");
        for i in 0..parsed.vec.len() {
            if i != 0 {
                print!(",");
            }

            print(&parsed.vec[i]);
        }
        print!("]");
    } else {
        print!("{}", parsed.num);
    }
}

fn in_correct_order(parsedl: &VecOrNum, parsedr: &VecOrNum) -> TrueFalseOrNoDecision {
    let result: TrueFalseOrNoDecision;

    if !parsedl.is_vec && !parsedr.is_vec {
        if parsedl.num < parsedr.num {
            result = TrueFalseOrNoDecision::True;
        } else if parsedl.num > parsedr.num {
            result = TrueFalseOrNoDecision::False;
        } else {
            result = TrueFalseOrNoDecision::NoDecision;
        }
    } else if parsedl.is_vec && !parsedr.is_vec {
        let new_struct = VecOrNum {
            is_vec: true,
            vec: vec![VecOrNum {
                is_vec: false,
                vec: Vec::new(),
                num: parsedr.num,
            }],
            num: 0,
        };

        result = in_correct_order(parsedl, &new_struct);
    } else if !parsedl.is_vec && parsedr.is_vec {
        let new_struct = VecOrNum {
            is_vec: true,
            vec: vec![VecOrNum {
                is_vec: false,
                vec: Vec::new(),
                num: parsedl.num,
            }],
            num: 0,
        };

        result = in_correct_order(&new_struct, parsedr);
    } else {
        // both are vecs
        let mut i = 0;

        loop {
            if i >= parsedl.vec.len() && i >= parsedr.vec.len() {
                result = TrueFalseOrNoDecision::NoDecision;
                break;
            } else if i >= parsedl.vec.len() {
                result = TrueFalseOrNoDecision::True;
                break;
            } else if i >= parsedr.vec.len() {
                result = TrueFalseOrNoDecision::False;
                break;
            }

            let left = &parsedl.vec[i];
            let right = &parsedr.vec[i];

            let intermediate_result = in_correct_order(left, right);

            if intermediate_result == TrueFalseOrNoDecision::True
                || intermediate_result == TrueFalseOrNoDecision::False
            {
                result = intermediate_result;
                break;
            }

            i += 1;
        }
    }

    println!("COMPARED:");
    print(&parsedl);
    println!();
    print(&parsedr);
    println!();
    println!("CORRECT: {:?}", result);
    println!();

    return result;
}
