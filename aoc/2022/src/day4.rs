use std::path::PathBuf;

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    fn new_from_str(line: &str) -> Self {
        let split_data = line.trim().split("-").collect::<Vec<&str>>();
        Self {
            start: u32::from_str_radix(split_data[0], 10).unwrap(),
            end: u32::from_str_radix(split_data[1], 10).unwrap(),
        }
    }

    /// This function only works for positive numbers
    fn fully_contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

fn day4_part1(data: &Vec<(Range, Range)>) -> usize {
    data.iter()
        .filter(|(first, second)| first.fully_contains(second) || second.fully_contains(first))
        .count()
}

fn parse_values_from_file(path: &PathBuf) -> Vec<(Range, Range)> {
    let input = common::read_file(path);
    input
        .trim()
        .split("\n")
        .map(|line| {
            let split_line = line.trim().split(",").collect::<Vec<&str>>();
            (
                Range::new_from_str(split_line[0]),
                Range::new_from_str(split_line[1]),
            )
        })
        .collect::<Vec<(Range, Range)>>()
}

pub fn day4_part1_solution(path: &PathBuf) -> String {
    let parsed_data = parse_values_from_file(path);
    day4_part1(&parsed_data).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse_input() -> Vec<(Range, Range)> {
        let input = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

        let parsed_data = input
            .trim()
            .split("\n")
            .map(|line| {
                let split_line = line.trim().split(",").collect::<Vec<&str>>();
                (
                    Range::new_from_str(split_line[0]),
                    Range::new_from_str(split_line[1]),
                )
            })
            .collect::<Vec<(Range, Range)>>();
        parsed_data
    }

    #[test]
    fn test_day4_part1() {
        {
            let first = Range::new(6, 6);
            let second = Range::new(4, 6);
            assert!(second.fully_contains(&first));
        }

        {
            let first = Range::new(2, 8);
            let second = Range::new(3, 7);
            assert!(!second.fully_contains(&first));
            assert!(first.fully_contains(&second));
        }

        let parsed_data = test_parse_input();
        assert_eq!(day4_part1(&parsed_data), 2);
    }
}
