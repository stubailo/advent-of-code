use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
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

    let non_zero_valves: HashSet<String> = HashSet::from_iter(
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

    let mut queue: VecDeque<(Vec<String>, u64, u64, u64)> = VecDeque::new();

    queue.push_back((vec!["AA".to_string()], 0, 0, 0));

    let mut max_final_pressure = 0 as u64;
    let mut final_path: Vec<String> = vec![];

    loop {
        let next = queue.pop_front();

        if next.is_none() {
            break;
        }

        let (current_path, current_time, current_valve_pressure, total_pressure) = next.unwrap();

        let current_valve = current_path.last().unwrap();

        for (valve, distance) in distances.get(current_valve).unwrap() {
            let new_time = current_time + distance + 1;

            if current_path.contains(valve) || new_time > 30 {
                let final_pressure = total_pressure + current_valve_pressure * (30 - current_time);
                if final_pressure > max_final_pressure {
                    max_final_pressure = final_pressure;
                    final_path = current_path.clone();
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

    println!("Max final pressure is: {}", max_final_pressure);
    println!("Final path is: {:?}", final_path);

    println!("Time elapsed is: {:?}", start.elapsed());
}
