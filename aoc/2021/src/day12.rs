use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

#[derive(Debug, Clone)]
struct Cave {
    name: String,
    big: bool,
}

impl Cave {
    fn new(name: String) -> Self {
        let big = name.chars().all(|x| x.is_uppercase());
        Self { name, big }
    }
}

struct Graph<'a> {
    path: Vec<Cave>,
    allowed_paths: &'a HashMap<String, HashSet<String>>,
    allowed_revisit: u32,
}

impl<'a> Graph<'a> {
    fn new(path: Vec<Cave>, allowed_paths: &'a HashMap<String, HashSet<String>>) -> Self {
        Self {
            path,
            allowed_paths,
            allowed_revisit: 0,
        }
    }

    fn new_with_allowed_revisit(
        path: Vec<Cave>,
        allowed_paths: &'a HashMap<String, HashSet<String>>,
        allowed_revisit: u32,
    ) -> Self {
        Self {
            path,
            allowed_paths,
            allowed_revisit,
        }
    }

    fn already_visited(&self, cave: &Cave) -> bool {
        self.path.iter().find(|x| x.name == cave.name).is_some()
    }

    fn get_cave_name_path(&self) -> String {
        self.path
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>()
            .join(",")
    }

    /// Step 2: Create a next API (similar to simulation)
    /// Iter 1
    /// start -> A
    /// start -> b

    /// Iter 2
    /// start -> A -> c
    /// start -> A -> b
    /// start -> A -> end

    /// start -> b -> d
    /// start -> b -> A
    /// satrt -> b -> end

    /// Iter 3
    /// ...
    fn compute_next_paths(&mut self) -> Option<Vec<Vec<Cave>>> {
        let mut cave_paths: Vec<Vec<Cave>> = Vec::new();

        let current_cave = self.path.last().unwrap();

        if current_cave.name == "end" {
            // NOTE, If current_cave == "end" then we return None
            None
        } else {
            // NOTE, We append the new cave location to the end of self.path
            let allowed_paths_from_current_cave =
                self.allowed_paths.get(&current_cave.name).unwrap();

            // println!(
            //     "Current: {:?}, Allowed: {:?} Revisit: {}",
            //     self.get_cave_name_path(),
            //     allowed_paths_from_current_cave,
            //     self.allowed_revisit,
            // );

            for cave_name in allowed_paths_from_current_cave {
                let cave_to_go = Cave::new(cave_name.clone());

                // Small cave &&
                // Cave has already been visited
                let add_cave = if !cave_to_go.big && self.already_visited(&cave_to_go) {
                    // We ignore this path
                    // for example: start -> b -> d -> b (wrong)
                    false
                } else {
                    // Add cave path
                    true
                };

                if add_cave {
                    let mut new_cave_path = self.path.clone();
                    new_cave_path.push(cave_to_go);
                    cave_paths.push(new_cave_path);
                }
            }

            Some(cave_paths)
        }
    }
}

fn parse_values_from_file(path: &PathBuf) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    // Elimination conditions
    // Note, We do not want anyone going from a cave to "start"
    // Note, We do not want anyone going from "end" to a cave
    let mut add_to_map_fn = |key: &String, value: &String| {
        if value != "start" && key != "end" {
            if map.contains_key(key) {
                map.get_mut(key).unwrap().insert(value.clone());
            } else {
                map.insert(key.clone(), HashSet::from([value.clone()]));
            }
        }
    };

    common::read_file(path).trim().split("\n").for_each(|line| {
        let split_data = line.trim().split("-").collect::<Vec<&str>>();

        let first = split_data[0].to_string();
        let second = split_data[1].to_string();

        add_to_map_fn(&first, &second);
        add_to_map_fn(&second, &first);
    });

    map
}

fn day12_part1(allowed_paths: &HashMap<String, HashSet<String>>) -> u32 {
    let mut final_traversals = Vec::new();
    let mut full_traversal_counter = 0;

    let mut queue = Vec::new();
    let start_graph = Graph::new(vec![Cave::new("start".to_string())], &allowed_paths);
    queue.push(start_graph);

    while !queue.is_empty() {
        let mut current_graph = queue.pop().unwrap();
        let paths = current_graph.compute_next_paths();

        if paths.is_none() {
            // We have reached the end
            final_traversals.push(current_graph.get_cave_name_path());
            full_traversal_counter += 1;
        } else {
            // We have not reached the end
            for next_path in paths.unwrap() {
                let new_graph = Graph::new(next_path, &allowed_paths);
                queue.push(new_graph);
            }
        }
    }

    // final_traversals.len()
    // println!("Final Traversal: {:?}", final_traversals);
    full_traversal_counter
}

pub fn day12_part1_solution(path: &PathBuf) -> String {
    let allowed_paths = parse_values_from_file(path);
    day12_part1(&allowed_paths).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse(data: &[&str]) -> HashMap<String, HashSet<String>> {
        let mut map: HashMap<String, HashSet<String>> = HashMap::new();
        data.iter().for_each(|d| {
            // Split the string with -
            // [first, second]
            let split_data = d
                .split("-")
                .map(|x| x.trim().to_string())
                .collect::<Vec<String>>();

            // Elimination conditions
            // Note, We do not want anyone going from a cave to "start"
            // Note, We do not want anyone going from "end" to a cave

            let first = split_data[0].clone();
            let second = split_data[1].clone();
            // Add map[first] = second
            if second != "start" && first != "end" {
                if map.contains_key(&first) {
                    map.get_mut(&first).unwrap().insert(second.clone());
                } else {
                    map.insert(first.clone(), HashSet::from([second.clone()]));
                }
            }

            // Add map[second] = first
            if first != "start" && second != "end" {
                if map.contains_key(&second) {
                    map.get_mut(&second).unwrap().insert(first.clone());
                } else {
                    map.insert(second.clone(), HashSet::from([first.to_string()]));
                }
            }
        });
        map
    }

    #[test]
    fn test_day12_part1_example1() {
        let data = ["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];

        let allowed_paths = test_parse(&data);
        println!("Allowed_Paths: {:#?}", allowed_paths);
        assert_eq!(day12_part1(&allowed_paths), 10);
    }

    #[test]
    fn test_day12_part1_example2() {
        let data = [
            "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa",
            "kj-HN", "kj-dc",
        ];

        let allowed_paths = test_parse(&data);
        println!("Allowed_Paths: {:#?}", allowed_paths);
        assert_eq!(day12_part1(&allowed_paths), 19);
    }

    #[test]
    fn test_day12_part1_example3() {
        let data = [
            "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
            "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
        ];

        let allowed_paths = test_parse(&data);
        println!("Allowed_Paths: {:#?}", allowed_paths);
        assert_eq!(day12_part1(&allowed_paths), 226);
    }

    #[test]
    fn test_day12_part1_solution() {
        let solution = day12_part1_solution(&PathBuf::new().join("inputs").join("day12_input.txt"));
        println!("Solution: {}", solution);
    }
}
