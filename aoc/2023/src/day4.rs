use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<usize>,
    your_numbers: Vec<usize>,
}

impl Card {
    fn points(&self) -> usize {
        let won_numbers: usize = self
            .your_numbers
            .iter()
            .map(|c| {
                if self.winning_numbers.contains(c) {
                    1
                } else {
                    0
                }
            })
            .sum();

        if won_numbers == 0 {
            return 0;
        }
        let won_numbers = won_numbers - 1;
        2usize.pow(won_numbers as u32)
    }
}

fn parse_input(input: String) -> Vec<Card> {
    input
        .trim()
        .split('\n')
        .map(|l| {
            let card = l.split('|').collect::<Vec<&str>>();
            let winning_numbers = card[0].split_terminator(":").collect::<Vec<&str>>();
            let winning_numbers = winning_numbers[1]
                .trim()
                .split_ascii_whitespace()
                .map(|l| l.parse::<usize>().unwrap())
                .collect::<HashSet<usize>>();
            let your_numbers = card[1]
                .trim()
                .split_ascii_whitespace()
                .map(|l| l.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Card {
                winning_numbers,
                your_numbers,
            }
        })
        .collect::<Vec<Card>>()
}

pub fn day4_part1_solution(input: String) -> String {
    let cards = parse_input(input);
    let ans: usize = cards.iter().map(|c| c.points()).sum();
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_day4_part1() {
        let cards = parse_input(INPUT_STR.into());
        assert_eq!(cards[0].points(), 8);

        let ans = day4_part1_solution(INPUT_STR.into());
        assert_eq!(ans, "13");
    }
}
