pub fn day3_part1_solution(input: String) -> String {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let ans = re
        .captures_iter(&input)
        .map(|capture| {
            let a_str = capture.get(1).unwrap().as_str();
            let b_str = capture.get(2).unwrap().as_str();
            if (1..=3).contains(&a_str.len()) && (1..=3).contains(&b_str.len()) {
                a_str.parse::<u32>().unwrap() * b_str.parse::<u32>().unwrap()
            } else {
                0
            }
        })
        .sum::<u32>();
    ans.to_string()
}

pub fn day3_part2_solution(input: String) -> String {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();

    let mut ans = 0;
    let mut process = true;
    for capture in re.captures_iter(&input) {
        let identifier = capture.get(0).unwrap().as_str();
        if identifier.contains("mul") {
            if !process {
                continue;
            }
            let a_str = capture.get(1).unwrap().as_str();
            let b_str = capture.get(2).unwrap().as_str();
            let multiplied = if (1..=3).contains(&a_str.len()) && (1..=3).contains(&b_str.len()) {
                a_str.parse::<u32>().unwrap() * b_str.parse::<u32>().unwrap()
            } else {
                0
            };
            ans += multiplied;
        } else if identifier.contains("don't") {
            process = false;
        } else if identifier.contains("do") {
            process = true;
        } else {
            unreachable!()
        }
    }
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const INPUT_STR2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn test_part1() {
        let ans = day3_part1_solution(INPUT_STR.into());
        println!("Ans: {ans}");
        assert_eq!(ans, "161");
    }

    #[test]
    fn test_part2() {
        let ans = day3_part2_solution(INPUT_STR2.into());
        println!("Ans: {ans}");
        assert_eq!(ans, "48");
    }
}
