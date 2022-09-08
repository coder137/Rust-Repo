use std::path::PathBuf;

enum LineType {
    Good,
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn get_line_info(data: &str) -> LineType {
    let match_closure = |current_value: Option<char>, matched_value: char| -> bool {
        let success = match current_value {
            Some(data) => {
                let mut success = false;
                if data == matched_value {
                    success = true;
                }
                success
            }
            None => false,
        };
        success
    };

    let mut matcher = Vec::new();
    let mut error_value: Option<char> = None;
    for ch in data.chars() {
        let value = match ch {
            '(' | '[' | '{' | '<' => {
                matcher.push(ch);
                true
            }
            ')' => match_closure(matcher.pop(), '('),
            ']' => match_closure(matcher.pop(), '['),
            '}' => match_closure(matcher.pop(), '{'),
            '>' => match_closure(matcher.pop(), '<'),
            _ => true,
        };

        if !value {
            error_value = Some(ch);
            break;
        }
    }

    if error_value.is_some() {
        LineType::Corrupted(error_value.unwrap())
    } else {
        if !matcher.is_empty() {
            LineType::Incomplete(matcher)
        } else {
            LineType::Good
        }
    }
}

/// Get corresponding points if illegal character is detected
/// ) = 3
/// ] = 57
/// } = 1197
/// > = 25137
/// none = 0
fn get_point_if_illegal_character(data: &str) -> u32 {
    let line_type = get_line_info(data);

    let value = match line_type {
        LineType::Corrupted(data) => match data {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        },
        LineType::Good | LineType::Incomplete(_) => 0,
    };
    value
}

fn day10_part1(data: &Vec<String>) -> u32 {
    data.iter().map(|x| get_point_if_illegal_character(x)).sum()
}

fn parse_values_from_file(path: &PathBuf) -> Vec<String> {
    let data = common::read_file(path);
    data.trim()
        .split("\n")
        .map(|x| x.trim().to_string())
        .collect()
}

pub fn day10_part1_solution(path: &PathBuf) -> String {
    let data = parse_values_from_file(path);
    day10_part1(&data).to_string()
}

fn complete_incomplete_line(incomplete: &Vec<char>) -> u64 {
    incomplete
        .iter()
        .rev()
        .map(|ch| match ch {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("This should never happen! {}", ch),
        })
        // NOTE, It is very important to keep this as a u64
        .fold(0_u64, |acc, i| acc * 5 + i)
}

fn day10_part2(data: &Vec<String>) -> u64 {
    let mut incomplete_data = data
        .iter()
        .map(|x| match get_line_info(x) {
            LineType::Good | LineType::Corrupted(_) => 0,
            LineType::Incomplete(incomplete_data) => complete_incomplete_line(&incomplete_data),
        })
        .filter(|&x| x != 0)
        .collect::<Vec<u64>>();
    incomplete_data.sort();
    incomplete_data[incomplete_data.len().div_euclid(2)]
}

pub fn day10_part2_solution(path: &PathBuf) -> String {
    let parsed_data = parse_values_from_file(path);
    day10_part2(&parsed_data).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_part1() {
        let data = "{([(<{}[<>[]}>{[]{[(<()>";
        assert_eq!(get_point_if_illegal_character(data), 1197);

        let data = "[[<[([]))<([[{}[[()]]]";
        assert_eq!(get_point_if_illegal_character(data), 3);

        let data = "[{[{({}]{}}([{[{{{}}([]";
        assert_eq!(get_point_if_illegal_character(data), 57);

        let data = "[<(<(<(<{}))><([]([]()";
        assert_eq!(get_point_if_illegal_character(data), 3);

        let data = "<{([([[(<>()){}]>(<<{{";
        assert_eq!(get_point_if_illegal_character(data), 25137);

        let data = "[({(<(())[]>[[{[]{<()<>>";
        assert_eq!(get_point_if_illegal_character(data), 0);

        let data = [
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ];

        let ans: u32 = data.iter().map(|x| get_point_if_illegal_character(x)).sum();
        assert_eq!(ans, 26397);
    }

    #[test]
    fn test_day10_part2() {
        let data = [
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ]
        .to_vec();

        assert_eq!(day10_part2(&data), 288957);

        // 996439232
    }
}
