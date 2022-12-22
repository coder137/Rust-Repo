use std::{collections::HashSet, path::PathBuf};

fn create_hashset(from: &str) -> HashSet<char> {
    let mut hset = HashSet::new();
    from.chars().for_each(|c| {
        hset.insert(c);
    });
    hset
}

fn to_priority(character: char) -> u32 {
    if character >= 'a' && character <= 'z' {
        (character as u32 - 'a' as u32) + 1
    } else if character >= 'A' && character <= 'Z' {
        (character as u32 - 'A' as u32) + 27
    } else {
        0
    }
}

fn day3_part1(data: &Vec<String>) -> u32 {
    data.iter()
        .map(|line| {
            let len = line.len() / 2;
            // println!("First Half: {}", &line[..len]);
            // println!("Second Half: {}", &line[len..]);
            let first_hset = create_hashset(&line[..len]);
            let second_hset = create_hashset(&line[len..]);
            first_hset
                .iter()
                .map(|c| {
                    if second_hset.contains(&c) {
                        to_priority(*c)
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn parse_values_from_file(path: &PathBuf) -> Vec<String> {
    let input = common::read_file(path);
    input
        .trim()
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect()
}

pub fn day3_part1_solution(path: &PathBuf) -> String {
    let parsed_data = parse_values_from_file(path);
    day3_part1(&parsed_data).to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn test_input_parse() -> Vec<String> {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        input
            .trim()
            .split("\n")
            .map(|line| line.trim().to_string())
            .collect()
    }

    #[test]
    fn test_day3_part1() {
        let parsed_data = test_input_parse();
        let ans = parsed_data
            .iter()
            .map(|line| {
                let len = line.len() / 2;
                // println!("First Half: {}", &line[..len]);
                // println!("Second Half: {}", &line[len..]);
                let first_hset = create_hashset(&line[..len]);
                let second_hset = create_hashset(&line[len..]);

                first_hset
                    .iter()
                    .map(|c| {
                        if second_hset.contains(&c) {
                            to_priority(*c)
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum::<u32>();
        assert_eq!(ans, 157);

        assert_eq!(day3_part1(&parsed_data), 157);
    }
}
