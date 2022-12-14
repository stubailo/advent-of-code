use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut set_of_occupied_points: HashSet<(i32, i32)> = HashSet::new();
    let mut largest_y = 0;

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        let mut prev_pt_opt: Option<(i32, i32)> = Option::None;

        for pt_str in line.split(" -> ") {
            let pt: Vec<i32> = pt_str
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let pt = (pt[0], pt[1]);

            if pt.1 > largest_y {
                largest_y = pt.1;
            }

            if prev_pt_opt.is_none() {
                prev_pt_opt = Option::Some(pt);
                continue;
            }

            let prev_pt = prev_pt_opt.unwrap();

            let (x1, y1) = prev_pt;
            let (x2, y2) = pt;

            if x1 == x2 {
                let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

                for y in min_y..=max_y {
                    set_of_occupied_points.insert((x1, y));
                }
            } else {
                let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

                for x in min_x..=max_x {
                    set_of_occupied_points.insert((x, y1));
                }
            }

            prev_pt_opt = Option::Some(pt);
        }
    }

    let mut num_grains_of_sand = 0;

    loop {
        // simulate one grain of sand falling, and see if set of occupied points increases in size

        let mut sand_location = (500, 0);

        let mut is_last_grain = false;

        loop {
            let (x, y) = sand_location;

            if y > largest_y + 3 {
                panic!("Sand location is too high");
            }

            let mut found_next_loc = false;
            for next_loc in [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)] {
                if next_loc.1 < largest_y + 2 && !set_of_occupied_points.contains(&next_loc) {
                    sand_location = next_loc;
                    found_next_loc = true;
                    break;
                }
            }

            if !found_next_loc {
                // we have reached an end point
                set_of_occupied_points.insert(sand_location);

                // print sand_location
                num_grains_of_sand += 1;

                if sand_location == (500, 0) {
                    is_last_grain = true;
                }

                break;
            }
        }

        if is_last_grain {
            break;
        }
    }

    // print num grains
    println!("Num grains of sand: {}", num_grains_of_sand);

    println!("Time elapsed is: {:?}", start.elapsed());
}
