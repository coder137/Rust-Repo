use std::path::PathBuf;

fn calories_carried_by_each_elf(data: &Vec<Option<u32>>) -> Vec<u32> {
    let mut elves_calories = Vec::new();
    let mut current_calorie_counter = 0;
    data.iter().for_each(|d| match d {
        Some(value) => {
            current_calorie_counter += value;
        }
        None => {
            elves_calories.push(current_calorie_counter);
            current_calorie_counter = 0;
        }
    });
    if current_calorie_counter != 0 {
        elves_calories.push(current_calorie_counter);
    }
    elves_calories
}

fn day1_part1(data: &Vec<Option<u32>>) -> u32 {
    *calories_carried_by_each_elf(data).iter().max().unwrap()
}

fn parse_values_from_file(path: &PathBuf) -> Vec<Option<u32>> {
    let input = common::read_file(path);
    input
        .trim()
        .split("\n")
        .map(|l| {
            let parsed = l.trim().parse::<u32>();
            if parsed.is_ok() {
                Some(parsed.unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<Option<u32>>>()
}

pub fn day1_part1_solution(path: &PathBuf) -> String {
    let parsed_data = parse_values_from_file(path);
    day1_part1(&parsed_data).to_string()
}

fn day1_part2(data: &Vec<Option<u32>>) -> u32 {
    let mut calories_carried = calories_carried_by_each_elf(data);
    calories_carried.sort_by(|a, b| b.cmp(a));
    calories_carried.iter().take(3).sum()
}

pub fn day1_part2_solution(path: &PathBuf) -> String {
    let parsed_data = parse_values_from_file(path);
    day1_part2(&parsed_data).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse_input() -> Vec<Option<u32>> {
        let input = "1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000";

        let split_data = input
            .trim()
            .split("\n")
            .map(|l| {
                let parsed = l.trim().parse::<u32>();
                if parsed.is_ok() {
                    Some(parsed.unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<Option<u32>>>();
        split_data
    }

    #[test]
    fn test_day1_part1() {
        let parsed_data = test_parse_input();

        let mut elves_calories = Vec::new();
        let mut current_calorie_counter = 0;
        parsed_data.iter().for_each(|d| match d {
            Some(value) => {
                current_calorie_counter += value;
            }
            None => {
                elves_calories.push(current_calorie_counter);
                current_calorie_counter = 0;
            }
        });
        if current_calorie_counter != 0 {
            elves_calories.push(current_calorie_counter);
        }
        println!("Elves Calories: {:?}", elves_calories);
        let max_value = *elves_calories.iter().max().unwrap();
        assert_eq!(max_value, 24000);

        assert_eq!(day1_part1(&parsed_data), 24000);
    }

    #[test]
    fn test_day1_part1_solution() {
        let solution = day1_part1_solution(&PathBuf::new().join("inputs").join("day1_input.txt"));
        println!("Solution : {:?}", solution);
    }

    #[test]
    fn test_dayt1_part2() {
        let parsed_data = test_parse_input();
        assert_eq!(day1_part2(&parsed_data), 45000);
    }
}
