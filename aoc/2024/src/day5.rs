use std::collections::{hash_map::Entry, HashMap, HashSet};

// pub type Orderings = HashMap<u32, HashSet<u32>>;
pub type Sequences = Vec<Vec<u32>>;

pub struct Orderings(HashMap<u32, HashSet<u32>>);
impl Orderings {
    pub fn is_ordered(&self, sequence: &[u32]) -> bool {
        for index in 0..sequence.len() - 1 {
            let preceding = sequence[index];
            let following = &sequence[index + 1..];
            let orderings = self
                .0
                .get(&preceding)
                .unwrap_or_else(|| panic!("Must contain a ordering key {preceding}"));
            let is_ordered = following.iter().all(|f| orderings.contains(f));
            if !is_ordered {
                return false;
            }
        }
        true
    }

    fn reorder(&self, sequence: &mut [u32]) {
        for index in 0..sequence.len() - 1 {
            loop {
                let current = sequence[index];
                let following = &sequence[index + 1..];
                let orderings = self
                    .0
                    .get(&current)
                    .unwrap_or_else(|| panic!("Must contain a ordering key {current}"));
                let is_ordered = following.iter().all(|f| orderings.contains(f));
                // println!("{sequence:?} -> {current} {following:?} {is_ordered}");
                if !is_ordered {
                    sequence[index..].rotate_left(1);
                } else {
                    break;
                }
            }
        }
    }
}

fn parse_input(input: String) -> (Orderings, Sequences) {
    let mut orderings: HashMap<u32, HashSet<u32>> = HashMap::default();
    let mut data = input.trim().split("\n\n");
    data.next().unwrap().split("\n").for_each(|data| {
        let mut iter = data.split("|");
        let before = iter.next().unwrap().parse::<u32>().unwrap();
        let after = iter.next().unwrap().parse::<u32>().unwrap();
        match orderings.entry(before) {
            Entry::Occupied(mut oe) => {
                oe.get_mut().insert(after);
            }
            Entry::Vacant(ve) => {
                ve.insert(HashSet::from([after]));
            }
        }

        match orderings.entry(after) {
            Entry::Occupied(_oe) => {
                // Do nothing
            }
            Entry::Vacant(ve) => {
                ve.insert(HashSet::new());
            }
        }
    });

    let sequences = data
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            line.split(",")
                .map(|iline| iline.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (Orderings(orderings), sequences)
}

pub fn day5_part1_solution(input: String) -> String {
    let (orderings, sequences) = parse_input(input);
    let mut ans = 0;
    for s in sequences {
        let is_ordered = orderings.is_ordered(&s);
        if !is_ordered {
            continue;
        }
        let num = s[s.len() / 2];
        ans += num;
    }
    ans.to_string()
}

pub fn day5_part2_solution(input: String) -> String {
    let (orderings, sequences) = parse_input(input);
    let mut ans = 0;
    for mut s in sequences {
        let is_ordered = orderings.is_ordered(&s);
        if is_ordered {
            continue;
        }
        orderings.reorder(&mut s);
        let num = s[s.len() / 2];
        ans += num;
    }
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        let ans = day5_part1_solution(INPUT_STR.into());
        println!("Ans: {ans}");
        assert_eq!(ans, "143");
    }

    #[test]
    fn test_part2() {
        let ans = day5_part2_solution(INPUT_STR.into());
        println!("Ans: {ans}");
        assert_eq!(ans, "123");
    }
}
