use std::{collections::HashMap, path::PathBuf};

struct FishState {
    current_state: Vec<u32>,
}

impl FishState {
    fn new(state: Vec<u32>) -> Self {
        FishState {
            current_state: state,
        }
    }
}

impl Iterator for FishState {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut add_more: Vec<u32> = Vec::new();
        self.current_state.iter_mut().for_each(|x| {
            if *x == 0 {
                *x = 6;
                add_more.push(8);
            } else {
                *x -= 1;
            }
        });

        self.current_state.append(&mut add_more);
        Some(self.current_state.clone())
    }
}

fn simulate_days(initial_state: u32, remaining_days: usize) -> Vec<u32> {
    FishState::new(vec![initial_state])
        .nth(remaining_days)
        .unwrap()
}

fn day6_part1(state: Vec<u32>) -> String {
    FishState::new(state).nth(79).unwrap().len().to_string()
}

pub fn day6_part1_solution() -> String {
    let data = common::read_file(&PathBuf::new().join("day6").join("input.txt"));
    let state = data
        .trim()
        .split(',')
        .map(|x| u32::from_str_radix(x, 10).unwrap())
        .collect::<Vec<u32>>();
    day6_part1(state)
}

fn day6_part2(state: Vec<u32>) -> String {
    let hashmap = HashMap::from([
        (0, simulate_days(0, 127)),
        (1, simulate_days(1, 127)),
        (2, simulate_days(2, 127)),
        (3, simulate_days(3, 127)),
        (4, simulate_days(4, 127)),
        (5, simulate_days(5, 127)),
        (6, simulate_days(6, 127)),
        (7, simulate_days(7, 127)),
        (8, simulate_days(8, 127)),
    ]);

    // Example [3, 4, 3, 1, 2] -> [ 3 -> expanded 128 times]
    // [3 ... 127 times] take each time [expand another 128 times]
    state
        .iter()
        .map(|x| {
            hashmap
                .get(x)
                .unwrap()
                .iter()
                .map(|y| hashmap.get(y).unwrap().len())
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

pub fn day6_part2_solution() -> String {
    let data = common::read_file(&PathBuf::new().join("day6").join("input.txt"));
    let state = data
        .trim()
        .split(',')
        .map(|x| u32::from_str_radix(x, 10).unwrap())
        .collect::<Vec<u32>>();
    day6_part2(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_aoc_base() {
        let state = "3,4,3,1,2";
        let fishes = state
            .trim()
            .split(',')
            .map(|x| u32::from_str_radix(x, 10).unwrap())
            .collect::<Vec<u32>>();
        assert_eq!(fishes, vec![3, 4, 3, 1, 2]);

        assert_eq!(FishState::new(fishes.clone()).nth(17).unwrap().len(), 26);
        assert_eq!(FishState::new(fishes.clone()).nth(79).unwrap().len(), 5934);

        let hashmap = HashMap::from([
            (0, simulate_days(0, 127).len()),
            (1, simulate_days(1, 127).len()),
            (2, simulate_days(2, 127).len()),
            (3, simulate_days(3, 127).len()),
            (4, simulate_days(4, 127).len()),
            (5, simulate_days(5, 127).len()),
            (6, simulate_days(6, 127).len()),
            (7, simulate_days(7, 127).len()),
            (8, simulate_days(8, 127).len()),
        ]);

        let ans = FishState::new(fishes.clone())
            .nth(127)
            .unwrap()
            .iter()
            .map(|x| *hashmap.get(x).unwrap())
            .sum::<usize>();
        assert_eq!(ans, 26984457539);
    }

    #[test]
    fn day6_part2_test() {
        let state = "3,4,3,1,2";
        let fishes = state
            .trim()
            .split(',')
            .map(|x| u32::from_str_radix(x, 10).unwrap())
            .collect::<Vec<u32>>();
        assert_eq!(fishes, vec![3, 4, 3, 1, 2]);

        assert_eq!(day6_part2(fishes), "26984457539");
    }
}
