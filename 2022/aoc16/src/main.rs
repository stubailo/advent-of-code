use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::ops::Sub;
use std::time::Instant;

struct Valve {
    name: String,
    flow_rate: u64,
    next_valves: Vec<String>,
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut valves: HashMap<String, Valve> = HashMap::new();

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        let mut tokens = line.split([' ', '=', ';', ',']);

        // we need items 1, 5, 11, 13, 15, etc

        let valve = tokens.nth(1).unwrap();
        let flow_rate = tokens.nth(3).unwrap();
        let next_valves: Vec<&str> = tokens.skip(5).step_by(2).collect();

        let valve = Valve {
            name: valve.to_string(),
            flow_rate: flow_rate.parse::<u64>().unwrap(),
            next_valves: next_valves.iter().map(|s| s.to_string()).collect(),
        };

        valves.insert(valve.name.clone(), valve);
    }

    let mut non_zero_valves: HashSet<String> = HashSet::from_iter(
        valves
            .iter()
            .filter(|(_, v)| v.flow_rate > 0 || v.name == "AA")
            .map(|(k, _)| k.to_string()),
    );

    // find the distance from each valve to each other non-zero valve
    let mut distances: HashMap<String, HashMap<String, u64>> = HashMap::new();

    for name in non_zero_valves.clone() {
        let mut distance_map: HashMap<String, u64> = HashMap::new();
        let mut valves_seen: HashSet<String> = HashSet::new();

        let mut queue: VecDeque<(String, u64)> = VecDeque::new();
        queue.push_back((name.to_owned(), 0));

        while queue.len() > 0 {
            let (current_valve_name, distance) = queue.pop_front().unwrap();

            let current_valve = valves.get(&current_valve_name).unwrap();

            for next_valve in &current_valve.next_valves {
                if valves_seen.contains(next_valve) {
                    continue;
                }

                if non_zero_valves.contains(next_valve) {
                    distance_map.insert(next_valve.to_string(), distance + 1);
                }

                valves_seen.insert(next_valve.to_string());
                queue.push_back((next_valve.to_string(), distance + 1));
            }
        }

        distances.insert(name.to_string(), distance_map);
    }

    non_zero_valves.remove("AA");

    // OK so here's my trick --
    // the amount of pressure is totally independent between me and the elephant
    // so I go to some locations, and the elephant goes to a different set of locations, and
    // we just add up the result

    // so I reduced part 1 to part 2 by dividing all of the valves that have non-zero pressure
    // into all possible combinations of two sets of valves, and giving each subset to me or
    // the elephant and then just running the part 1 solution and adding it up

    // my trick for part 1 which was important for this to run fast enough is to pre-generate
    // a graph of distances between all valves that have non-zero flow, and ignore the ones that
    // have zero flow completely

    // generate all subsets of non_zero_values into a vector
    let mut subsets: Vec<HashSet<String>> = Vec::new();

    for i in 0..(1 << non_zero_valves.len()) {
        let mut subset: HashSet<String> = HashSet::new();

        for (j, valve) in non_zero_valves.iter().enumerate() {
            if i & (1 << j) != 0 {
                subset.insert(valve.to_string());
            }
        }

        subsets.push(subset);
    }

    println!("Number of subsets is: {}", subsets.len());

    let mut max_final_pressure = 0 as u64;

    let mut i = 0;

    // print all subsets
    for subset in &subsets {
        let max_final_pressure1 = get_best_pressure(&valves, &distances, &subset);
        let max_final_pressure2 =
            get_best_pressure(&valves, &distances, &non_zero_valves.sub(&subset));

        let final_pressure = max_final_pressure1 + max_final_pressure2;

        if final_pressure > max_final_pressure {
            max_final_pressure = final_pressure;
        }

        i += 1;

        if (i % 1000) == 0 {
            println!("At index: {}", i);
        }
    }

    println!("Max final pressure is: {}", max_final_pressure);

    println!("Time elapsed is: {:?}", start.elapsed());
}

fn get_best_pressure(
    valves: &HashMap<String, Valve>,
    distances: &HashMap<String, HashMap<String, u64>>,
    allowed_valves: &HashSet<String>,
) -> u64 {
    let mut queue: VecDeque<(Vec<String>, u64, u64, u64)> = VecDeque::new();

    queue.push_back((vec!["AA".to_string()], 0, 0, 0));

    let mut max_final_pressure = 0 as u64;

    loop {
        let next = queue.pop_front();

        if next.is_none() {
            break;
        }

        let (current_path, current_time, current_valve_pressure, total_pressure) = next.unwrap();

        let current_valve = current_path.last().unwrap();

        for (valve, distance) in distances.get(current_valve).unwrap() {
            if !allowed_valves.contains(valve) {
                continue;
            }

            let new_time = current_time + distance + 1;

            if current_path.contains(valve) || new_time > 26 {
                let final_pressure = total_pressure + current_valve_pressure * (26 - current_time);
                if final_pressure > max_final_pressure {
                    max_final_pressure = final_pressure;
                }
                continue;
            }

            let mut new_path = current_path.clone();
            new_path.push(valve.to_string());

            let new_valve_pressure = current_valve_pressure + valves.get(valve).unwrap().flow_rate;

            let new_total_pressure = total_pressure + current_valve_pressure * (distance + 1);

            queue.push_back((new_path, new_time, new_valve_pressure, new_total_pressure));
        }
    }

    return max_final_pressure;
}
