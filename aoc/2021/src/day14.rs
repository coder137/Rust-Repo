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
    let element_counts = polymer_template.get_element_counts();

    let mut max = usize::MIN;
    let mut min = usize::MAX;
    element_counts.iter().for_each(|(_, value)| {
        let v = *value;
        max = v.max(max);
        min = v.min(min);
    });
    max.checked_sub(min).unwrap()
}

pub fn day14_part1_solution(path: &PathBuf) -> String {
    let (start, map) = parse_values_from_file(path);
    day14_part1(start, &map).to_string()
}

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
}
