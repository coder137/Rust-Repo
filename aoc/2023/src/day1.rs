use std::collections::HashMap;

fn parse_input(input: String) -> Vec<String> {
    input
        .trim()
        .split("\n")
        .map(|d| d.to_owned())
        .collect::<Vec<String>>()
}

pub fn day1_part1_solution(input: String) -> String {
    let parsed = parse_input(input);
    let ans = parsed
        .iter()
        .map(|l| {
            let parsed = l
                .chars()
                .filter_map(|d| {
                    let digit = d.to_digit(10);
                    digit
                })
                .collect::<Vec<u32>>();
            assert!(!parsed.is_empty());
            let number = parsed.first().unwrap() * 10 + parsed.last().unwrap();
            number
        })
        .sum::<u32>();
    ans.to_string()
}

fn find_all(line: &String, map: &HashMap<&str, u32>) -> Vec<u32> {
    let mut start = 0;
    let mut end = start + 1;

    let mut stored_numbers = Vec::new();

    loop {
        // exit condition
        if end == line.len() + 1 {
            break;
        }

        let current_slice = &line[start..end];

        // Check if the slice is a number
        if current_slice.len() == 1 {
            let parsed = current_slice.parse::<u32>();
            if parsed.is_ok() {
                stored_numbers.push(parsed.unwrap());
                start += 1;
                end = start + 1;
                continue;
            }
        }

        let is_matching = map.keys().filter(|d| d.starts_with(current_slice)).count() != 0;
        let matched = map.keys().filter(|d| **d == current_slice).count() != 0;

        match (matched, is_matching) {
            (true, true) => {
                let num = *map.get(current_slice).unwrap();
                stored_numbers.push(num);
                start += 1;
                end = start + 1;
            }
            (true, false) => {
                unreachable!("")
            }
            (false, true) => {
                end += 1;
            }
            (false, false) => {
                start += 1;
                end = start + 1;
            }
        }
    }
    stored_numbers
}

pub fn day1_part2_solution(input: String) -> String {
    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let parsed = parse_input(input);
    let ans = parsed
        .iter()
        .map(|l| {
            let data = find_all(l, &map);
            data.first().unwrap() * 10 + data.last().unwrap()
        })
        // .inspect(|d| println!("{d}"))
        .sum::<u32>();
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";

    const INPUT_STR2: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

    #[test]
    fn test_solution1() {
        let parsed = parse_input(INPUT_STR.into());
        println!("parsed: {:?}", parsed);
        let ans = day1_part1_solution(INPUT_STR.into());
        assert_eq!(ans, "142");
    }

    #[test]
    fn test_solution2() {
        let parsed = parse_input(INPUT_STR2.into());
        println!("parsed: {:?}", parsed);
        let map = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);

        let mut line = "two1nine".to_owned();
        map.iter().for_each(|(key, value)| {
            line = line.replace(key, value.to_string().as_str());
        });
        assert_eq!(line, "219");

        let line = "178ncllbfkkh4eightwoq".to_owned();
        let found_numbers = find_all(&line, &map);
        assert_eq!(found_numbers, vec![1, 7, 8, 4, 8, 2]);

        let line = "4nineeightseven2".to_owned();
        let found_numbers = find_all(&line, &map);
        assert_eq!(found_numbers, vec![4, 9, 8, 7, 2]);

        let ans = day1_part2_solution(INPUT_STR2.into());
        assert_eq!(ans, "281");
    }
}
