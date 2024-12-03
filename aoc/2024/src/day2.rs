fn parse_input(input: String) -> Vec<Vec<u32>> {
    input
        .trim()
        .split('\n')
        .map(|d| {
            d.trim()
                .split(" ")
                .map(|d| d.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn is_safe(level: &[u32]) -> bool {
    let compare_is_decreasing = level[0] > level[1];
    level.windows(2).all(|d| {
        let is_decreasing = d[0] > d[1];
        let same = compare_is_decreasing == is_decreasing;
        let diff = d[0].abs_diff(d[1]);
        (1..=3).contains(&diff) && same
    })
}

pub fn day2_part1_solution(input: String) -> String {
    let input = parse_input(input);
    let ans = input
        .into_iter()
        .map(|i| if is_safe(&i) { 1 } else { 0 })
        .sum::<u32>();
    ans.to_string()
}

pub fn day2_part2_solution(input: String) -> String {
    let input = parse_input(input);
    let ans = input
        .into_iter()
        .map(|mut i| {
            if is_safe(&i) {
                1
            } else {
                for index in 0..i.len() {
                    let element = i.remove(index);
                    if is_safe(&i) {
                        return 1;
                    }
                    i.insert(index, element);
                }
                0
            }
        })
        .sum::<u32>();
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_day1_part1() {
        let parsed = parse_input(INPUT_STR.into());
        println!("parsed: {:?}", parsed);
        let ans = day2_part1_solution(INPUT_STR.into());
        println!("Ans: {ans}");
        assert_eq!(ans, "2");
    }

    #[test]
    fn test_day1_part2() {
        let ans = day2_part2_solution(INPUT_STR.into());
        println!("Ans: {ans}");
        assert_eq!(ans, "4");
    }
}
