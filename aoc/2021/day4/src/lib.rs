use std::{collections::HashMap, path::PathBuf};

#[derive(Hash, Eq, PartialEq, Debug)]
struct BingoInfo {
    index: (usize, usize),
    visited: bool,
}

impl BingoInfo {
    fn new(index: (usize, usize)) -> Self {
        BingoInfo {
            index,
            visited: false,
        }
    }
}

#[derive(Debug)]
pub struct BingoCard {
    numbers: Vec<Vec<u32>>, // NOTE, We still need this from tracking win and lose conditions
    numbers_by_index: HashMap<u32, BingoInfo>,
    win_rounds: usize,
    won_at_number: u32,
}

impl BingoCard {
    fn new(numbers: Vec<Vec<u32>>) -> Self {
        let mut numbers_by_index: HashMap<u32, BingoInfo> = HashMap::new();
        numbers.iter().enumerate().for_each(|(vindex, hor)| {
            hor.iter().enumerate().for_each(|(hindex, value)| {
                numbers_by_index.insert(*value, BingoInfo::new((vindex, hindex)));
            })
        });

        BingoCard {
            numbers,
            numbers_by_index,
            win_rounds: 0,
            won_at_number: 0,
        }
    }

    fn add(&mut self, num: u32) {
        if let Some(value) = self.numbers_by_index.get_mut(&num) {
            value.visited = true;
        }
    }

    fn won(&self) -> bool {
        let mut won = self.check_won(self.numbers[0].iter());
        won = won || self.check_won(self.numbers[1].iter());
        won = won || self.check_won(self.numbers[2].iter());
        won = won || self.check_won(self.numbers[3].iter());
        won = won || self.check_won(self.numbers[4].iter());

        won = won || self.check_won(self.create_iter_at_index(0));
        won = won || self.check_won(self.create_iter_at_index(1));
        won = won || self.check_won(self.create_iter_at_index(2));
        won = won || self.check_won(self.create_iter_at_index(3));
        won = won || self.check_won(self.create_iter_at_index(4));
        won
    }

    fn compute_minimum_win_rounds(&mut self, rounds: &Vec<u32>) {
        rounds.iter().enumerate().any(|(index, value)| {
            self.add(*value);
            let won = self.won();
            if won {
                self.win_rounds = index;
                self.won_at_number = *value;
            }
            won
        });
    }

    fn compute_winning_product(&self, winning_number: u32) -> u32 {
        self.numbers_by_index
            .iter()
            .map(
                |(number, info)| {
                    if info.visited == true {
                        0
                    } else {
                        *number
                    }
                },
            )
            .sum::<u32>()
            * winning_number
    }

    // PRIVATE
    fn check_won<'a>(&self, mut iter: impl Iterator<Item = &'a u32>) -> bool {
        iter.all(|x| self.numbers_by_index[x].visited == true)
    }

    fn create_iter_at_index(&self, collect_i: usize) -> impl Iterator<Item = &u32> {
        assert!(collect_i <= 4);
        self.numbers
            .iter()
            .flat_map(|v| v)
            .enumerate()
            .filter(move |(index, _)| index % 5 == collect_i)
            .map(|x| x.1)
    }
}

fn parse_values_from_file() -> (Vec<u32>, Vec<BingoCard>) {
    let read = common::read_file(&PathBuf::new().join("day4").join("input.txt"));
    let split = read.trim().split("\n").collect::<Vec<&str>>();

    // *
    let numbers = split
        .iter()
        .nth(0)
        .map(|x| x.trim())
        .unwrap()
        .split(',')
        .map(|x| u32::from_str_radix(x, 10).unwrap())
        .collect::<Vec<u32>>();
    // println!("{:?}", numbers);

    // *
    let mut bingo_cards = Vec::new();

    let mut iter = split.iter().skip(1);
    let mut curr = iter
        .by_ref()
        .skip(1)
        .take(5)
        .map(|x| x.trim())
        .collect::<Vec<&str>>();
    while curr.len() == 5 {
        let bingo_numbers = curr
            .iter()
            .map(|x| {
                x.split_whitespace()
                    .map(|y| u32::from_str_radix(y, 10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();
        // println!("{:?}", bingo_numbers);
        bingo_cards.push(BingoCard::new(bingo_numbers));

        curr = iter
            .by_ref()
            .skip(1)
            .take(5)
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
    }
    // println!("{:?}", bingo_cards.len());
    (numbers, bingo_cards)
}

fn day4_part1(numbers: Vec<u32>, mut bingo_cards: Vec<BingoCard>) -> String {
    let mut winning_value = None;
    for num in numbers {
        bingo_cards.iter_mut().any(|card| {
            card.add(num);
            let won = card.won();
            if won {
                winning_value = Some(card.compute_winning_product(num));
            }
            won
        });
        match winning_value {
            Some(_) => {
                break;
            }
            None => {}
        }
    }
    winning_value.unwrap().to_string()
}

pub fn day4_part1_solution() -> String {
    let (numbers, bingo_cards) = parse_values_from_file();
    day4_part1(numbers, bingo_cards)
}

fn day4_part2(numbers: Vec<u32>, mut bingo_cards: Vec<BingoCard>) -> String {
    let max_win_rounds = bingo_cards
        .iter_mut()
        .map(|card| {
            card.compute_minimum_win_rounds(&numbers);
            card.win_rounds
        })
        .max()
        .unwrap();

    bingo_cards
        .iter()
        .filter(|card| card.win_rounds == max_win_rounds)
        .map(|card| card.compute_winning_product(card.won_at_number))
        .sum::<u32>()
        .to_string()
}

pub fn day4_part2_solution() -> String {
    let (numbers, bingo_cards) = parse_values_from_file();
    day4_part2(numbers, bingo_cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo_card() {
        let bingo_card = [
            [22, 13, 17, 11, 0].to_vec(),
            [8, 2, 23, 4, 24].to_vec(),
            [21, 9, 14, 16, 7].to_vec(),
            [6, 10, 3, 18, 5].to_vec(),
            [1, 12, 20, 15, 19].to_vec(),
        ]
        .to_vec();

        let card = BingoCard::new(bingo_card);
        assert_eq!(card.numbers_by_index[&22].index, (0, 0));
        assert_eq!(card.numbers_by_index[&13].index, (0, 1));
        assert_eq!(card.numbers_by_index[&8].index, (1, 0));
        assert_eq!(card.numbers_by_index[&2].index, (1, 1));
        assert_eq!(card.numbers_by_index[&6].index, (3, 0));

        assert_eq!(
            card.create_iter_at_index(0)
                .map(|x| *x)
                .collect::<Vec<u32>>(),
            vec![22, 8, 21, 6, 1]
        );

        assert_eq!(
            card.create_iter_at_index(2)
                .map(|x| *x)
                .collect::<Vec<u32>>(),
            vec![17, 23, 14, 3, 20]
        );
    }

    #[test]
    fn test_win_conditions() {
        let selected_nums = [
            7_u32, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
            19, 3, 26, 1,
        ];

        let bingo_card_nums = [
            [14, 21, 17, 24, 4].to_vec(),
            [10, 16, 15, 9, 19].to_vec(),
            [18, 8, 23, 26, 20].to_vec(),
            [22, 11, 13, 6, 5].to_vec(),
            [2, 0, 12, 3, 7].to_vec(),
        ]
        .to_vec();

        let mut card = BingoCard::new(bingo_card_nums);

        let mut winning_number = 0;
        for num in selected_nums {
            card.add(num);
            if card.won() {
                winning_number = num;
                break;
            }
        }
        assert_eq!(winning_number, 24);
        assert_eq!(card.compute_winning_product(winning_number), 4512);
    }
}
