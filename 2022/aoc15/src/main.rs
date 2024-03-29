use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    // x, y, distance
    let mut sensors: Vec<(i32, i32, i32)> = Vec::new();

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        // parse 4 numbers from a string like "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
        let strs: Vec<&str> = line.split([' ', ',', '=', ':'].as_ref()).collect();

        let sx = strs[3].parse::<i32>().unwrap();
        let sy = strs[6].parse::<i32>().unwrap();
        let bx = strs[13].parse::<i32>().unwrap();
        let by = strs[16].parse::<i32>().unwrap();

        // calculate manhattan distance
        let distance = (sx - bx).abs() + (sy - by).abs();

        sensors.push((sx, sy, distance));
    }

    let mut points_to_check: HashSet<(i32, i32)> = HashSet::new();

    let max_x_y = 4000000;

    // print how many sensors
    println!("Sensors: {}", sensors.len());

    // find all x, y locations that are just 1 distance farther from a sensor
    for sensor in &sensors {
        for x in (sensor.0 - sensor.2 - 1)..=(sensor.1 + sensor.2 + 1) {
            if x < 0 || x > max_x_y {
                continue;
            }

            let spread = ((sensor.0 - x).abs() - (sensor.2 + 1)).abs();

            let y1 = sensor.1 + spread;
            let y2 = sensor.1 - spread;

            if y1 >= 0 && y1 <= max_x_y {
                points_to_check.insert((x, y1));
            }
            if y2 >= 0 && y2 <= max_x_y {
                points_to_check.insert((x, y2));
            }
        }
    }

    // print number of points to check
    println!("Points to check: {}", points_to_check.len());

    let mut points_checked = 0;

    for (x, y) in points_to_check {
        let mut within_sensor_distance = false;

        points_checked += 1;

        for sensor in &sensors {
            let distance = (sensor.0 - x).abs() + (sensor.1 - y).abs();

            if distance <= sensor.2 {
                within_sensor_distance = true;
                break;
            }
        }

        if !within_sensor_distance {
            println!("({}, {})", x, y);
            let result = (x as u64) * 4000000 + (y as u64);
            println!("Result: {}", result);
            break;
        }
    }

    println!("Points checked: {}", points_checked);

    println!("Time elapsed is: {:?}", start.elapsed());
}
