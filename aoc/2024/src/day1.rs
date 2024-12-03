use std::collections::{hash_map::Entry, HashMap};

fn parse_input(input: String) -> Vec<(u32, u32)> {
    input
        .trim()
        .split('\n')
        .map(|d| {
            let mut iter = d.trim().split(" ").filter(|d2| !d2.is_empty());
            let d1 = iter.next().unwrap();
            let d1 = d1.parse::<u32>().unwrap();
            let d2 = iter.next().unwrap();
            let d2 = d2.parse::<u32>().unwrap();
            (d1, d2)
        })
        .collect::<Vec<_>>()
}

pub fn day1_part1_solution(input: String) -> String {
    let input = parse_input(input);
    let (mut alist, mut blist): (Vec<_>, Vec<_>) = input.into_iter().unzip();
    alist.sort();
    blist.sort();

    let ans = alist
        .into_iter()
        .zip(blist)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>();
    ans.to_string()
}

pub fn day1_part2_solution(input: String) -> String {
    let input = parse_input(input);
    let (alist, blist): (Vec<_>, Vec<_>) = input.into_iter().unzip();

    let mut bmap = HashMap::<u32, u32>::default();
    for b in blist {
        match bmap.entry(b) {
            Entry::Occupied(mut oe) => {
                let v = oe.get_mut();
                *v += 1;
            }
            Entry::Vacant(ve) => {
                ve.insert(1);
            }
        }
    }
    let ans = alist
        .into_iter()
        .map(|a| {
            let value = match bmap.get(&a) {
                Some(value) => *value,
                None => 0,
            };
            value * a
        })
        .sum::<u32>();

    ans.to_string()
}

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

    #[test]
    fn test_day1_part2() {
        let ans = day1_part2_solution(INPUT_STR.into());
        println!("Ans: {ans}");
    }
}
