use std::{collections::HashMap, path::PathBuf};

fn sort(data: &str) -> String {
    let mut char_iter = String::from(data).chars().collect::<Vec<char>>();
    char_iter.sort();
    String::from_iter(char_iter)
}

struct Pattern {
    unique_pattern: Vec<String>,
    output_value: Vec<String>,
}

impl<'a> Pattern {
    fn new(data: &str) -> Self {
        let parsed_data = data
            .trim()
            .split('|')
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let unique_pattern = parsed_data[0]
            .split(' ')
            .map(|x| sort(x))
            .collect::<Vec<String>>();
        let output_value = parsed_data[1]
            .split(' ')
            .map(|x| sort(x))
            .collect::<Vec<String>>();
        Pattern {
            unique_pattern,
            output_value,
        }
    }

    // Count the unique alphabets and give a number between 0-9
    // Number 1: Has 2 letters
    // Number 4: Has 4 letters
    // Number 7: Has 3 letters
    // Number 8: Has 7 letters
    fn get_unique_number_from_str(data: &str) -> Option<u8> {
        match data.len() {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        }
    }

    fn deduce_output_value(&self) -> u32 {
        let numbers = self.deduce_numbers_from_unique_pattern();

        self.output_value
            .iter()
            .map(|x| {
                let s = sort(x);
                numbers.get(&s).unwrap()
            })
            .fold(0_u32, |acc, &elem| acc * 10 + elem as u32)
    }

    fn deduce_numbers_from_unique_pattern(&self) -> HashMap<&String, u8> {
        let mut hashmap_by_strkey = HashMap::new();
        let mut hashmap_by_numkey = HashMap::new();
        self.unique_pattern.iter().for_each(|x| {
            let value = Pattern::get_unique_number_from_str(x);
            match value {
                Some(v) => {
                    hashmap_by_strkey.insert(x, v);
                    hashmap_by_numkey.insert(v, x);
                }
                None => {}
            }
        });

        let str1 = *hashmap_by_numkey.get(&1).unwrap();
        let str4 = *hashmap_by_numkey.get(&4).unwrap();
        // 6 len: 0, 6, 9
        // 5 len: 2, 3, 5

        // Get 3, filter by len 5, remove all 1's overlap -> 3 len (3)
        // Get 2, filter by len 5, remove all 4's overlap -> 3 len (3)
        // Get 5 (remaining is 5)
        let get_all_len5 = self
            .unique_pattern
            .iter()
            .filter(|x| x.len() == 5)
            .collect::<Vec<&String>>();

        let str3 = get_all_len5
            .iter()
            .filter(|x| {
                x.chars()
                    .filter(|y| !str1.contains(*y))
                    .collect::<Vec<char>>()
                    .len()
                    == 3
            })
            .map(|x| *x)
            .reduce(|_, y| y)
            .unwrap();
        hashmap_by_strkey.insert(str3, 3);

        let str2 = get_all_len5
            .iter()
            .filter(|x| {
                x.chars()
                    .filter(|y| !str4.contains(*y))
                    .collect::<Vec<char>>()
                    .len()
                    == 3
            })
            .map(|x| *x)
            .reduce(|_, y| y)
            .unwrap();
        hashmap_by_strkey.insert(str2, 2);

        let str5 = get_all_len5
            .iter()
            .filter(|&&y| y != str2 && y != str3)
            .map(|x| *x)
            .reduce(|_, y| y)
            .unwrap();
        hashmap_by_strkey.insert(str5, 5);

        // Get 6, filter by len 6, remove all 1's overlap -> 5 len
        // Get 9, filter by len 6, remove all 4's overlap -> 2 len
        // Get 0 (remaining is 0)
        let get_all_len6 = self
            .unique_pattern
            .iter()
            .filter(|x| x.len() == 6)
            .collect::<Vec<&String>>();

        let str6 = get_all_len6
            .iter()
            .filter(|x| {
                x.chars()
                    .filter(|y| !str1.contains(*y))
                    .collect::<Vec<char>>()
                    .len()
                    == 5
            })
            .map(|x| *x)
            .reduce(|_, y| y)
            .unwrap();
        hashmap_by_strkey.insert(str6, 6);

        let str9 = get_all_len6
            .iter()
            .filter(|x| {
                x.chars()
                    .filter(|y| !str4.contains(*y))
                    .collect::<Vec<char>>()
                    .len()
                    == 2
            })
            .map(|x| *x)
            .reduce(|_, y| y)
            .unwrap();
        hashmap_by_strkey.insert(str9, 9);

        let str0 = get_all_len6
            .iter()
            .filter(|&&x| x != str6 && x != str9)
            .map(|x| *x)
            .reduce(|_, y| y)
            .unwrap();
        hashmap_by_strkey.insert(str0, 0);

        hashmap_by_strkey
    }
}

fn parse_values_from_file() -> Vec<Pattern> {
    let data = common::read_file(&PathBuf::new().join("day8").join("input.txt"));

    data.trim()
        .split("\n")
        .map(|x| Pattern::new(x))
        .collect::<Vec<Pattern>>()
}

fn day8_part1(data: &Vec<Pattern>) -> u32 {
    data.iter()
        .map(|x| {
            x.output_value
                .iter()
                .map(|y| {
                    let ans = Pattern::get_unique_number_from_str(y);
                    match ans {
                        Some(_) => 1,
                        None => 0,
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

pub fn day8_part1_solution() -> String {
    let parsed_data = parse_values_from_file();
    day8_part1(&parsed_data).to_string()
}

fn day8_part2(data: &Vec<Pattern>) -> u32 {
    data.iter().map(|x| x.deduce_output_value()).sum()
}

pub fn day8_part2_solution() -> String {
    let parsed_data = parse_values_from_file();
    day8_part2(&parsed_data).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_part1_test() {
        let data = [
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
            fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
            fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
            cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
            efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
            gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
            gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
            cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
            ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
            gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
            fgae cfgab fg bagce",
        ];

        let parsed_data = data
            .iter()
            .map(|x| Pattern::new(x))
            .collect::<Vec<Pattern>>();

        assert_eq!(parsed_data[0].unique_pattern[0], "be");
        assert_eq!(parsed_data[0].output_value[0], "fdgacbe");

        // Number 1: Has 2 letters
        // Number 4: Has 4 letters
        // Number 7: Has 3 letters
        // Number 8: Has 7 letters
        assert_eq!(Pattern::get_unique_number_from_str("fdgacbe").unwrap(), 8);
        assert_eq!(Pattern::get_unique_number_from_str("gcbe").unwrap(), 4);

        assert_eq!(day8_part1(&parsed_data), 26);
    }

    #[test]
    fn day8_part2_test() {
        let data = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
        cdfeb fcadb cdfeb cdbaf";

        let pattern = Pattern::new(data);
        let map = pattern.deduce_numbers_from_unique_pattern();

        // println!("Data: {:?}", map);

        assert_eq!(map.get(&sort("acedgfb")).unwrap(), &8);
        assert_eq!(map.get(&sort("cdfbe")).unwrap(), &5);
        assert_eq!(map.get(&sort("gcdfa")).unwrap(), &2);
        assert_eq!(map.get(&sort("fbcad")).unwrap(), &3);
        assert_eq!(map.get(&sort("dab")).unwrap(), &7);
        assert_eq!(map.get(&sort("cefabd")).unwrap(), &9);
        assert_eq!(map.get(&sort("cdfgeb")).unwrap(), &6);
        assert_eq!(map.get(&sort("eafb")).unwrap(), &4);
        assert_eq!(map.get(&sort("cagedb")).unwrap(), &0);
        assert_eq!(map.get(&sort("ab")).unwrap(), &1);

        assert_eq!(pattern.deduce_output_value(), 5353);

        let data = [
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
            fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
            fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
            cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
            efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
            gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
            gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
            cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
            ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
            gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
            fgae cfgab fg bagce",
        ];

        let parsed_data = data
            .iter()
            .map(|x| Pattern::new(x))
            .collect::<Vec<Pattern>>();

        assert_eq!(day8_part2(&parsed_data), 61229);
    }
}
