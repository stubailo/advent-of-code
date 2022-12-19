use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::hash::Hash;
use std::hash::Hasher;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Robot {
    makes: Material,
    needs: Vec<(Material, i32)>,
}

fn serialize_materials_hashmap(materials: &HashMap<Material, i32>) -> Vec<(Material, i32)> {
    let mut materials_vec = Vec::<(Material, i32)>::new();

    for (material, amount) in materials.iter() {
        materials_vec.push((*material, *amount));
    }

    return materials_vec;
}

fn serialize_robots(robots: &Vec<&Robot>) -> Vec<Material> {
    let mut robots_vec = Vec::<Material>::new();

    for robot in robots.iter() {
        robots_vec.push(robot.makes.to_owned());
    }

    robots_vec.sort();

    return robots_vec;
}

fn main() {
    let start = Instant::now();
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut iter = contents.lines();

    let mut blueprints = Vec::<Vec<Robot>>::new();
    let num_minutes = 32;

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        let line = line.unwrap();

        let mut numbers = Vec::<i32>::new();
        let tokens = line.split([' ', ':', '.']).collect::<Vec<&str>>();

        for token in tokens {
            let number = token.parse::<i32>();

            if number.is_ok() {
                numbers.push(number.unwrap());
            }
        }

        let mut blueprint = Vec::<Robot>::new();

        blueprint.push(Robot {
            makes: Material::Ore,
            needs: vec![(Material::Ore, numbers[1])],
        });

        blueprint.push(Robot {
            makes: Material::Clay,
            needs: vec![(Material::Ore, numbers[2])],
        });

        blueprint.push(Robot {
            makes: Material::Obsidian,
            needs: vec![(Material::Ore, numbers[3]), (Material::Clay, numbers[4])],
        });

        blueprint.push(Robot {
            makes: Material::Geode,
            needs: vec![
                (Material::Ore, numbers[5]),
                (Material::Obsidian, numbers[6]),
            ],
        });

        blueprints.push(blueprint);
    }

    let mut product_of_blueprint_geodes = 1;
    for (blueprint_index, blueprint) in blueprints.iter().enumerate() {
        println!("Blueprint {}", blueprint_index);

        // now, DP our way to the optimal solution
        // state is:
        // 1. the current set of robots
        // 2. the current amount of each material
        // 3. the current step
        let mut queue =
            VecDeque::<(Vec<&Robot>, HashMap<Material, i32>, HashSet<Material>, i32)>::new();
        let mut visited = HashSet::<u64>::new();

        queue.push_back((vec![&blueprint[0]], HashMap::new(), HashSet::new(), 0));

        let mut max_geodes = 0;

        let mut last_step = 0;

        let mut max_geodes_per_step = HashMap::<i32, i32>::new();

        loop {
            let state = queue.pop_front();

            if state.is_none() {
                break;
            }

            let (robots, materials, robots_to_not_build, step) = state.unwrap();

            // serialize the state into a vector of integers
            let mut hasher = DefaultHasher::new();
            serialize_robots(&robots).hash(&mut hasher);
            serialize_materials_hashmap(&materials).hash(&mut hasher);
            let hash = hasher.finish();

            if visited.contains(&hash) {
                continue;
            }

            if last_step != step {
                print!("{} ", step);
            }

            // if last_step != step {
            //     println!("Current step: {}", step);

            //     // print num_per_step sorted
            //     let mut num_per_step_vec = Vec::<(i32, i32)>::new();

            //     for (step, num) in num_per_step.iter() {
            //         num_per_step_vec.push((*step, *num));
            //     }

            //     num_per_step_vec.sort_by(|a, b| a.0.cmp(&b.0));

            //     println!("NUM ITEMS");
            //     for (step, num) in num_per_step_vec.iter() {
            //         print!("{}: {}, ", step, num);
            //     }
            //     println!();

            //     // print max_geodes_per_step sorted
            //     let mut max_geodes_per_step_vec = Vec::<(i32, i32)>::new();

            //     for (step, num) in max_geodes_per_step.iter() {
            //         max_geodes_per_step_vec.push((*step, *num));
            //     }

            //     max_geodes_per_step_vec.sort_by(|a, b| a.0.cmp(&b.0));

            //     println!("NUM OBSIDIAN");
            //     for (step, num) in max_geodes_per_step_vec.iter() {
            //         print!("{}: {}, ", step, num);
            //     }
            //     println!();

            //     println!(
            //         "Max obsidian for last step: {}",
            //         max_geodes_per_step.get(&last_step).unwrap_or(&0)
            //     );
            // }
            last_step = step;

            let obsidian_for_this_step = materials.get(&Material::Geode).unwrap_or(&0);
            let max_obsidian_for_last_step = max_geodes_per_step.get(&last_step).unwrap_or(&0);

            if step > 24 && (*obsidian_for_this_step < max_obsidian_for_last_step / 5 * 4) {
                continue;
            }

            // num_per_step.insert(step, num_per_step.get(&step).unwrap_or(&0) + 1);

            if materials.get(&Material::Geode).unwrap_or(&0)
                > &max_geodes_per_step.get(&step).unwrap_or(&0)
            {
                max_geodes_per_step.insert(step, *materials.get(&Material::Geode).unwrap_or(&0));
            }

            // for robot in robots.iter() {
            //     print!("{:?} ", robot.makes);
            // }
            // println!();
            // for (material, amount) in materials.iter() {
            //     print!("{:?}: {} ", material, amount);
            // }
            // println!();

            if step == num_minutes {
                if materials.get(&Material::Geode).unwrap_or(&0) > &max_geodes {
                    // println!(
                    //     "Found a new max geodes: {}",
                    //     materials.get(&Material::Geode).unwrap_or(&0)
                    // );
                    max_geodes = *materials.get(&Material::Geode).unwrap_or(&0);
                }
                continue;
            }

            // the next steps are:
            // do nothing
            // build one of the robots

            let mut new_robots_to_not_build = robots_to_not_build.clone();
            let mut made_geode = false;

            for robot in blueprint.iter().rev() {
                if robots_to_not_build.contains(&robot.makes) {
                    continue;
                }

                if robot.makes == Material::Ore {
                    // count the number of ore robots
                    let mut num_ore_robots = 0;

                    for robot in robots.iter() {
                        if robot.makes == Material::Ore {
                            num_ore_robots += 1;
                        }
                    }

                    if num_ore_robots >= 4 {
                        continue;
                    }
                }

                let mut can_build = true;

                for (material, amount) in robot.needs.iter() {
                    let current_amount = materials.get(material).unwrap_or(&0);

                    if current_amount < amount {
                        can_build = false;
                        break;
                    }
                }

                if can_build {
                    let mut new_materials = materials.clone();
                    let mut new_robots = robots.clone();

                    for (material, amount) in robot.needs.iter() {
                        let current_amount = materials.get(material).unwrap_or(&0);

                        new_materials.insert(*material, current_amount - amount);
                    }

                    // use old robots
                    add_materials(&robots, &mut new_materials);

                    new_robots.push(robot);

                    queue.push_back((new_robots, new_materials, HashSet::new(), step + 1));

                    new_robots_to_not_build.insert(robot.makes);

                    if robot.makes == Material::Geode {
                        made_geode = true;
                        break;
                    }
                }
            }

            if made_geode {
                continue;
            }

            let mut new_materials = materials.clone();

            add_materials(&robots, &mut new_materials);

            queue.push_back((robots, new_materials, new_robots_to_not_build, step + 1));
            visited.insert(hash);
        }

        println!(
            "Max geodes for blueprint {}: {}",
            blueprint_index, max_geodes
        );

        product_of_blueprint_geodes *= max_geodes;
    }

    println!("Product: {}", product_of_blueprint_geodes);

    println!("Time elapsed is: {:?}", start.elapsed());
}

fn add_materials(robots: &Vec<&Robot>, materials: &mut HashMap<Material, i32>) {
    for robot in robots.iter() {
        let current_amount = materials.get(&robot.makes).unwrap_or(&0);

        materials.insert(robot.makes, current_amount + 1);
    }
}

fn print_blueprints(blueprints: &Vec<Vec<Robot>>) {
    for blueprint in blueprints.iter() {
        for robot in blueprint.iter() {
            print!("{:?} needs ", robot.makes);

            for (material, amount) in robot.needs.iter() {
                print!("{} {:?} ", amount, material);
            }

            println!();
        }

        println!();
    }
}
