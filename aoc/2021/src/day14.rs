use std::{collections::HashMap, path::PathBuf};

struct PolymerTemplate<'a> {
    start: String,
    map: &'a HashMap<String, char>,
}

impl<'a> PolymerTemplate<'a> {
    fn new(start: String, map: &'a HashMap<String, char>) -> Self {
        Self { start, map }
    }

    fn get_element_counts(&self) -> HashMap<char, usize> {
        let mut count_map: HashMap<char, usize> = HashMap::new();

        self.start.chars().for_each(|a| {
            if count_map.contains_key(&a) {
                let v = count_map.get_mut(&a).unwrap();
                *v += 1;
            } else {
                count_map.insert(a, 1);
            }
        });
        count_map
    }

    fn get_diff(&self) -> usize {
        let mut max = usize::MIN;
        let mut min = usize::MAX;
        self.get_element_counts().iter().for_each(|(_, value)| {
            let v = *value;
            max = v.max(max);
            min = v.min(min);
        });
        max.checked_sub(min).unwrap()
    }
}

impl<'a> Iterator for PolymerTemplate<'a> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let mut counter = 0;
        let mut vec = Vec::new();
        (0..self.start.len() - 1).for_each(|index| {
            let key = &self.start[index..index + 2];
            match self.map.get(key) {
                Some(value) => {
                    vec.push((index + 1 + counter, value));
                    counter += 1;
                }
                None => {}
            }
        });

        for (index, value) in vec {
            self.start.insert(index, value.clone());
        }
        Some(())
    }
}

fn parse_values_from_file(path: &PathBuf) -> (String, HashMap<String, char>) {
    let data = common::read_file(path);
    let mut split_iter = data.trim().split("\n");
    let start = split_iter
        .by_ref()
        .take(1)
        .map(|s| s.trim())
        .collect::<Vec<&str>>()[0]
        .to_owned();
    // println!("Start: {:?}", start);

    let mut map = HashMap::new();
    split_iter
        .by_ref()
        .skip(1)
        .map(|line| line.trim().split(" -> ").collect::<Vec<&str>>())
        .for_each(|line| {
            let key = line[0].to_owned();
            let value = line[1].chars().last().unwrap(); // There must only be ONE char here after parsing
            map.insert(key, value);
        });
    // println!("Map: {:?}", map);

    (start, map)
}

fn day14_part1(start: String, map: &HashMap<String, char>) -> usize {
    let mut polymer_template = PolymerTemplate::new(start, &map);
    polymer_template.nth(9).unwrap();
    polymer_template.get_diff()
}

pub fn day14_part1_solution(path: &PathBuf) -> String {
    let (start, map) = parse_values_from_file(path);
    day14_part1(start, &map).to_string()
}

/// ChainPolymerTemplate is a more efficient implementation of PolymerTemplate
/// Problem with PolymerTemplate: For higher iterations (nth computation) of PolymerTemplate, the memory allocations would be very high + computations would be slower
/// ChainPolymerTemplate exploits the fact that the given `map` of byte[2] -> byte[1] is added whenever an adjacent pair of byte[2] is found in a string
/// This creates another byte[2] pair which repeats over time
/// For example: NNCB can be broken up into {NN:1, NC:1, CB:1}
/// First Iteration NCNBCHB {NC:1, CN:1, NB:1, BC:1, CH:1, HB:1}
/// Second Iteration: NBCCNBBBCBHCB {CC:1, NB:2, HC:1, CN:1, BB:2, BC:2, BH:1, CB:2}
/// ... etc
/// We can now expand this very easily for the next iteration and store it more efficiently (count value instead of a large string)
/// We can also keep a count map to count 'chars: usize' seperately to solve the problem
struct ChainPolymerTemplate<'a> {
    chain: HashMap<String, usize>,
    map: &'a HashMap<String, char>,
    count: HashMap<char, usize>,
}

impl<'a> ChainPolymerTemplate<'a> {
    fn new(start: String, map: &'a HashMap<String, char>) -> Self {
        // Initialize chain
        // For example: {NN: 1, NC: 1, CB: 1}
        let mut chain = HashMap::new();
        (0..start.len() - 1)
            .map(|index| {
                let slice = &start[index..index + 2];
                slice.to_string()
            })
            .for_each(|data| {
                if chain.contains_key(&data) {
                    let value = chain.get_mut(&data).unwrap();
                    *value += 1;
                } else {
                    chain.insert(data, 1);
                }
            });

        // Initialize count
        // For example: {N:2, C:1, B:1}
        let mut count = HashMap::new();
        start.chars().for_each(|c| {
            if count.contains_key(&c) {
                let value = count.get_mut(&c).unwrap();
                *value += 1;
            } else {
                count.insert(c, 1);
            }
        });
        Self { chain, map, count }
    }

    fn get_diff(&self) -> usize {
        let mut max = usize::MIN;
        let mut min = usize::MAX;
        self.count.iter().for_each(|(_, value)| {
            let v = *value;
            max = v.max(max);
            min = v.min(min);
        });
        max.checked_sub(min).unwrap()
    }
}

impl<'a> Iterator for ChainPolymerTemplate<'a> {
    type Item = ();

    // There might be multiple chains (NN, NC, CB etc)
    fn next(&mut self) -> Option<Self::Item> {
        // For every next iteration
        // Check the map for new "char" to be added in between
        // Update the chain with the new chars added
        // Increment the chain and the count maps

        let mut new_chain = HashMap::new();
        let mut new_maps_insert = |key: String, amount: usize| {
            if new_chain.contains_key(&key) {
                let value = new_chain.get_mut(&key).unwrap();
                *value += amount;
            } else {
                new_chain.insert(key, amount);
            }
        };

        let mut count_insert = |ch: char, amount: usize| {
            if self.count.contains_key(&ch) {
                let value = self.count.get_mut(&ch).unwrap();
                *value += amount;
            } else {
                self.count.insert(ch, amount);
            }
        };

        self.chain
            .iter()
            .filter(|&(chain_key, _)| self.map.contains_key(chain_key))
            .for_each(|(chain_key, chain_value)| {
                let map_value = self.map.get(chain_key).unwrap();
                count_insert(map_value.clone(), chain_value.clone());

                let ch1 =
                    chain_key.chars().take(1).last().unwrap().to_string() + &map_value.to_string();
                let ch2 = map_value.to_string() + &chain_key.chars().last().unwrap().to_string();
                new_maps_insert(ch1, chain_value.clone());
                new_maps_insert(ch2, chain_value.clone());
            });

        // println!("Chain: {:?}", self.chain);
        // println!("New Maps: {:?}", new_chain);
        // println!("Count: {:?}", self.count);
        // println!("-------------------------");
        self.chain = new_chain;

        Some(())
    }
}

fn day14_part2(start: String, map: &HashMap<String, char>) -> usize {
    0
}

pub fn day14_part2_solution() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse() -> (String, HashMap<String, char>) {
        let data = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        let mut split_iter = data.trim().split("\n");
        let start = split_iter
            .by_ref()
            .take(1)
            .map(|s| s.trim())
            .collect::<Vec<&str>>()[0]
            .to_owned();
        println!("Start: {:?}", start);

        let mut map = HashMap::new();
        split_iter
            .by_ref()
            .skip(1)
            .map(|line| line.trim().split(" -> ").collect::<Vec<&str>>())
            .for_each(|line| {
                let key = line[0].to_owned();
                let value = line[1].chars().last().unwrap();
                map.insert(key, value);
            });
        println!("Map: {:?}", map);
        (start, map)
    }

    #[test]
    fn test_day14_part1() {
        let (start, map) = test_parse();
        let solution = day14_part1(start.clone(), &map);
        assert_eq!(solution, 1588);

        let solution = day14_part1_solution(&PathBuf::new().join("inputs").join("day14_input.txt"));
        println!("Solution: {}", solution);
    }

    #[test]
    fn test_day14_part1_method2() {
        let (start, map) = test_parse();
        let mut chain_polymer_template = ChainPolymerTemplate::new(start.clone(), &map);
        chain_polymer_template.nth(9);
        assert_eq!(chain_polymer_template.get_diff(), 1588);
    }

    #[test]
    fn test_day14_part2() {
        let (start, map) = test_parse();
        let mut chain_polymer_template = ChainPolymerTemplate::new(start.clone(), &map);
        chain_polymer_template.nth(39);
        assert_eq!(chain_polymer_template.get_diff(), 2188189693529);
    }
}
