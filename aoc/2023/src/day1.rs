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

#[cfg(test)]
mod tests {
    use super::parse_input;

    const INPUT_STR: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";

    #[test]
    fn test_solution1() {
        let parsed = parse_input(INPUT_STR.into());
        println!("parsed: {:?}", parsed);
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
        assert_eq!(ans, 142);
    }
}
