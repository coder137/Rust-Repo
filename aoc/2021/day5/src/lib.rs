use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
struct Range {
    p1: (u32, u32),
    p2: (u32, u32),
}

impl Range {
    fn new(p1: (u32, u32), p2: (u32, u32)) -> Self {
        Range { p1, p2 }
    }

    fn line_is_vertical(&self) -> bool {
        if self.p1.0 == self.p2.0 {
            true
        } else {
            false
        }
    }

    fn line_is_horizontal(&self) -> bool {
        if self.p1.1 == self.p2.1 {
            true
        } else {
            false
        }
    }

    fn get_list(&self) -> Vec<(u32, u32)> {
        let mut rvec = vec![self.p1];
        let mut next_num = self.next_number(self.p1);
        while next_num != None {
            let current_num = next_num.unwrap();
            rvec.push(current_num);
            next_num = self.next_number(current_num);
        }
        rvec
    }

    // PRIVATE
    fn next_number(&self, from: (u32, u32)) -> Option<(u32, u32)> {
        let x = from.0;
        let y = from.1;

        let to_x = self.p2.0;
        let to_y = self.p2.1;

        let mut opt = None;
        if from != self.p2 {
            let new_x: u32;
            let new_y: u32;
            if x == to_x {
                new_x = x;
            } else if x > to_x {
                new_x = x - 1;
            } else {
                // x < to_x
                new_x = x + 1;
            }

            if y == to_y {
                new_y = y;
            } else if y > to_y {
                new_y = y - 1;
            } else {
                // y < to_y
                new_y = y + 1;
            }
            opt = Some((new_x, new_y))
        }
        opt
    }
}

fn parse_values_from_file(path: &PathBuf) -> Vec<Range> {
    let data = common::read_file(path);
    data.trim()
        .split('\n')
        .map(|m| m.trim().split("->").collect::<Vec<&str>>())
        .map(|x| {
            let p1 = x[0]
                .trim()
                .split(',')
                .map(|z| u32::from_str_radix(z, 10).unwrap())
                .collect::<Vec<u32>>();
            let p2 = x[1]
                .trim()
                .split(',')
                .map(|z| u32::from_str_radix(z, 10).unwrap())
                .collect::<Vec<u32>>();
            Range::new((p1[0], p1[1]), (p2[0], p2[1]))
        })
        .collect::<Vec<Range>>()
}

fn day5_part1(ranges: &Vec<Range>) -> String {
    let mut hashmap: HashMap<(u32, u32), u32> = HashMap::new();

    ranges
        .iter()
        .filter(|x| x.line_is_horizontal() || x.line_is_vertical())
        .for_each(|x| {
            x.get_list().iter().for_each(|y| match hashmap.get_mut(y) {
                Some(value) => {
                    *value += 1;
                }
                None => {
                    hashmap.insert(*y, 1);
                }
            })
        });

    hashmap
        .iter()
        .map(|(_key, value)| {
            // println!("{:?} -> {}", _key, value);
            if *value > 1 {
                1
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn day5_part1_solution(path: &PathBuf) -> String {
    let ranges = parse_values_from_file(path);
    day5_part1(&ranges)
}

fn day5_part2(ranges: &Vec<Range>) -> String {
    let mut hashmap: HashMap<(u32, u32), u32> = HashMap::new();

    ranges.iter().for_each(|x| {
        x.get_list().iter().for_each(|y| match hashmap.get_mut(y) {
            Some(value) => {
                *value += 1;
            }
            None => {
                hashmap.insert(*y, 1);
            }
        })
    });

    hashmap
        .iter()
        .map(|(_key, value)| {
            // println!("{:?} -> {}", _key, value);
            if *value > 1 {
                1
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn day5_part2_solution(path: &PathBuf) -> String {
    let ranges = parse_values_from_file(path);
    day5_part2(&ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        let data = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
        "
        .trim();
        let _split = data
            .split('\n')
            .map(|m| m.trim().split("->").collect::<Vec<&str>>())
            .map(|x| {
                let p1 = x[0]
                    .trim()
                    .split(',')
                    .map(|z| u32::from_str_radix(z, 10).unwrap())
                    .collect::<Vec<u32>>();
                let p2 = x[1]
                    .trim()
                    .split(',')
                    .map(|z| u32::from_str_radix(z, 10).unwrap())
                    .collect::<Vec<u32>>();
                Range::new((p1[0], p1[1]), (p2[0], p2[1]))
            })
            .collect::<Vec<Range>>();
        // println!("{:?}", split);
    }

    #[test]
    fn range_get_list() {
        let range = Range::new((0, 9), (5, 9));
        assert_eq!(
            range.get_list(),
            vec![(0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9),]
        );

        let range = Range::new((8, 0), (0, 8));
        assert_eq!(
            range.get_list(),
            vec![
                (8, 0),
                (7, 1),
                (6, 2),
                (5, 3),
                (4, 4),
                (3, 5),
                (2, 6),
                (1, 7),
                (0, 8),
            ]
        );

        let range = Range::new((2, 2), (2, 0));
        assert_eq!(range.get_list(), vec![(2, 2), (2, 1), (2, 0),]);
    }

    #[test]
    fn range_itersection() {
        let ranges = vec![
            Range::new((0, 9), (5, 9)),
            Range::new((8, 0), (0, 8)),
            Range::new((9, 4), (3, 4)),
            Range::new((2, 2), (2, 1)),
            Range::new((7, 0), (7, 4)),
            Range::new((6, 4), (2, 0)),
            Range::new((0, 9), (2, 9)),
            Range::new((3, 4), (1, 4)),
            Range::new((0, 0), (8, 8)),
            Range::new((5, 5), (8, 2)),
        ];

        let mut hashmap: HashMap<(u32, u32), u32> = HashMap::new();

        ranges.iter().for_each(|x| {
            x.get_list().iter().for_each(|y| match hashmap.get_mut(y) {
                Some(value) => {
                    *value += 1;
                }
                None => {
                    hashmap.insert(*y, 1);
                }
            })
        });

        assert_eq!(hashmap.get(&(0, 9)), Some(&2));
        assert_eq!(hashmap.get(&(1, 9)), Some(&2));
        assert_eq!(hashmap.get(&(2, 9)), Some(&2));
        assert_eq!(hashmap.get(&(3, 4)), Some(&2));
        assert_eq!(hashmap.get(&(7, 4)), Some(&2));

        assert_eq!(day5_part1(&ranges), "5");
        assert_eq!(day5_part2(&ranges), "12");
    }
}
