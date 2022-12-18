use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut points: HashSet<(i32, i32, i32)> = HashSet::new();

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        // split line by , and parse each segment as an int
        let mut segments = line.split(",");
        let x: i32 = segments.next().unwrap().parse().unwrap();
        let y: i32 = segments.next().unwrap().parse().unwrap();
        let z: i32 = segments.next().unwrap().parse().unwrap();

        points.insert((x, y, z));
    }

    let points = &points;

    // for each starting point, run a space filling alg to find all points that are trapped

    // find min and max x, y, z
    let min_bounds: (i32, i32, i32) = points.iter().fold((0, 0, 0), |acc, (x, y, z)| {
        (
            if *x < acc.0 { *x } else { acc.0 },
            if *y < acc.1 { *y } else { acc.1 },
            if *z < acc.2 { *z } else { acc.2 },
        )
    });

    let max_bounds: (i32, i32, i32) = points.iter().fold((0, 0, 0), |acc, (x, y, z)| {
        (
            if *x > acc.0 { *x } else { acc.0 },
            if *y > acc.1 { *y } else { acc.1 },
            if *z > acc.2 { *z } else { acc.2 },
        )
    });

    let mut air_point_is_trapped: HashMap<(i32, i32, i32), bool> = HashMap::new();
    for (x, y, z) in points {
        // iterate over each of 6 directions
        for (dx, dy, dz) in &[
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let new_x = x + dx;
            let new_y = y + dy;
            let new_z = z + dz;

            if air_point_is_trapped.contains_key(&(new_x, new_y, new_z))
                || points.contains(&(new_x, new_y, new_z))
            {
                continue;
            }

            let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
            let mut points_visited: HashSet<(i32, i32, i32)> = HashSet::new();
            queue.push_back((new_x, new_y, new_z));

            let mut trapped = true;

            while !queue.is_empty() {
                let (x, y, z) = queue.pop_front().unwrap();

                if x < min_bounds.0
                    || x > max_bounds.0
                    || y < min_bounds.1
                    || y > max_bounds.1
                    || z < min_bounds.2
                    || z > max_bounds.2
                {
                    trapped = false;
                    break;
                }

                if points_visited.contains(&(x, y, z)) || points.contains(&(x, y, z)) {
                    continue;
                }

                if air_point_is_trapped.get(&(x, y, z)) == Some(&true) {
                    continue;
                }

                points_visited.insert((x, y, z));

                queue.push_back((x + 1, y, z));
                queue.push_back((x - 1, y, z));
                queue.push_back((x, y + 1, z));
                queue.push_back((x, y - 1, z));
                queue.push_back((x, y, z + 1));
                queue.push_back((x, y, z - 1));
            }

            air_point_is_trapped.insert((new_x, new_y, new_z), trapped);
        }
    }

    let mut num_unobstructed_faces = 0;
    for (x, y, z) in points {
        // iterate over each of 6 directions
        for (dx, dy, dz) in &[
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ] {
            let new_x = x + dx;
            let new_y = y + dy;
            let new_z = z + dz;

            if !points.contains(&(new_x, new_y, new_z))
                && air_point_is_trapped.get(&(new_x, new_y, new_z)) == Some(&false)
            {
                num_unobstructed_faces += 1;
            }
        }
    }

    println!(
        "Number of unobstructed faces is: {}",
        num_unobstructed_faces
    );

    println!("Time elapsed is: {:?}", start.elapsed());
}
