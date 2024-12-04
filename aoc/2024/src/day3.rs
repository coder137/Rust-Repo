// fn parse_input(input: String) -> Vec<Vec<u32>> {
//     input
//         .trim()
//         .split('\n')
//         .map(|d| {
//             d.trim()
//                 .split(" ")
//                 .map(|d| d.parse::<u32>().unwrap())
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>()
// }

// mul(x,y)
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

// pub fn day3_part2_solution(input: String) -> String {
//     let re = regex::Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\))").unwrap();
//     let ans = re.captures_iter(&input).map(|d| {});
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_day1_part1() {
        // let parsed = parse_input(INPUT_STR.into());
        // println!("parsed: {:?}", parsed);
        let ans = day3_part1_solution(INPUT_STR.into());
        println!("Ans: {ans}");
        // assert_eq!(ans, "2");
    }

    // #[test]
    // fn test_day1_part2() {
    //     let ans = day2_part2_solution(INPUT_STR.into());
    //     println!("Ans: {ans}");
    //     assert_eq!(ans, "4");
    // }
}
