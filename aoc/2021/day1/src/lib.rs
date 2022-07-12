use std::path::PathBuf;

extern crate common;

fn day1_part1(data: &Vec<&str>) -> String {
    data.windows(2)
        .map(|x| {
            let first = x[0].parse::<u32>().unwrap();
            let second = x[1].parse::<u32>().unwrap();
            if second > first {
                1
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn day1_part1_solution() -> String {
    let data = common::read_file(&PathBuf::new().join("day1").join("input.txt"));
    let split: Vec<&str> = data.trim().split("\n").collect();
    day1_part1(&split)
}

fn day1_part2(data: &Vec<&str>) -> String {
    let parsed = data
        .windows(3)
        .map(|x| x.iter().map(|x| x.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();
    parsed
        .windows(2)
        .map(|x| if x[1] > x[0] { 1 } else { 0 })
        .sum::<u32>()
        .to_string()
}

pub fn day1_part2_solution() -> String {
    let data = common::read_file(&PathBuf::new().join("day1").join("input.txt"));
    let split: Vec<&str> = data.trim().split("\n").collect();
    day1_part2(&split)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let counter = day1_part1(&vec!["0", "1"]);
        assert_eq!(counter, "1");

        let counter = day1_part1(&vec!["0", "2", "1"]);
        assert_eq!(counter, "1");

        let counter = day1_part1(&vec!["0", "2", "1", "2"]);
        assert_eq!(counter, "2");
    }

    #[test]
    fn test_day1_part2() {
        let counter = day1_part2(&vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ]);
        assert_eq!(counter, "5");
    }
}
