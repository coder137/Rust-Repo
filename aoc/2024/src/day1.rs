fn parse_input(input: String) -> Vec<Vec<String>> {
    input
        .trim()
        .split('\n')
        .map(|d| {
            d.trim()
                .split(" ")
                .filter(|d2| !d2.is_empty())
                .map(|d2| d2.to_owned())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn day1_part1_solution(input: String) -> String {
    let input = parse_input(input);
    let (mut alist, mut blist): (Vec<_>, Vec<_>) = input
        .into_iter()
        .map(|d| {
            let d1 = d[0].parse::<u32>().unwrap();
            let d2 = d[1].parse::<u32>().unwrap();
            (d1, d2)
        })
        .unzip();
    alist.sort();
    blist.sort();

    let ans = alist
        .into_iter()
        .zip(blist)
        .map(|(a, b)| (a).abs_diff(b))
        .sum::<u32>();
    ans.to_string()
}

// pub fn day1_part2_solution(input: String) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_day1_part1() {
        let parsed = parse_input(INPUT_STR.into());
        println!("parsed: {:?}", parsed);
        let ans = day1_part1_solution(INPUT_STR.into());
        println!("Ans: {ans}");
        // assert_eq!(ans, "142");
    }
}
