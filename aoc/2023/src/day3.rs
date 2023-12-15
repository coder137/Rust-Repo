#[derive(Debug)]
struct Number {
    num: usize,
    x_range: (usize, usize), // x_start, x_end
    y: usize,
}

struct Symbol {
    sym: char,
    pos: (usize, usize),
}

impl Number {
    fn is_part_number(&self, symbols: &Vec<Symbol>) -> bool {
        let left_x = self.x_range.0.checked_sub(1).unwrap_or(0);
        let right_x = self.x_range.1 + 1;
        let top_y = self.y.checked_sub(1).unwrap_or(0);
        let bottom_y = self.y + 1;
        symbols
            .iter()
            .filter(|s| {
                // NOTE, Symbol should be inside top_left, top_right, bottom_left or bottom_right bound
                // left_x <= x <= right_x;
                // top_y <= y <= bottom_y;
                let sym_x = s.pos.0;
                let sym_y = s.pos.1;
                left_x <= sym_x && sym_x <= right_x && top_y <= sym_y && sym_y <= bottom_y
            })
            .count()
            != 0
    }
}

fn extract_numbers_from_line(line: &str) -> Vec<(usize, (usize, usize))> {
    let mut front = 0;
    let mut back = front + 1;

    let mut numbers = Vec::new();
    loop {
        if back >= line.len() {
            break;
        }

        let current_slice = line[front..back].parse::<usize>();
        let next_slice = line[front..back + 1].parse::<usize>();

        // println!(
        //     "Slice: {:?} {:?} {back} {}",
        //     &line[front..back],
        //     &line[front..back + 1],
        //     line.len()
        // );
        match (current_slice.is_ok(), next_slice.is_ok()) {
            (true, true) => {
                // if next_slice is the last slice then we need to store the number
                if back + 1 == line.len() {
                    numbers.push((next_slice.unwrap(), (front, back)));
                }
                // Move next
                back += 1;
            }
            (true, false) => {
                numbers.push((current_slice.unwrap(), (front, back - 1)));
                front = back + 1;
                back = front + 1;
            }
            (false, true) => {
                // +, +3 for example can be parsed
                front += 1;
                back = front + 1;
            }
            (false, false) => {
                front += 1;
                back = front + 1;
            }
        }
    }
    numbers
}

fn extract_symbol_from_line(line: &str) -> Vec<(usize, char)> {
    line.chars()
        .enumerate()
        .filter(|&(_, c)| c != '.' && !c.is_numeric())
        .collect()
}

// TODO, Parsing the input is inefficient since we essentially doing a 2 pass
// One pass to extract numbers
// Second pass to extract symbols
// * Update algorithm to extract numbers + symbols in one pass
fn parse_input(input: String) -> (Vec<Number>, Vec<Symbol>) {
    let iter = input.trim().split('\n').enumerate();

    let numbers = iter
        .clone()
        .map(|(y, l)| {
            extract_numbers_from_line(l)
                .iter()
                .map(|&(num, x_range)| Number { num, x_range, y })
                .collect::<Vec<Number>>()
        })
        .flatten()
        .collect::<Vec<Number>>();

    let symbols = iter
        .map(|(y, l)| {
            extract_symbol_from_line(l)
                .iter()
                .map(|&(x, c)| Symbol {
                    sym: c,
                    pos: (x, y),
                })
                .collect::<Vec<Symbol>>()
        })
        .flatten()
        .collect::<Vec<Symbol>>();
    (numbers, symbols)
}

pub fn day3_part1_solution(input: String) -> String {
    // Get numbers and symbol locations
    let (numbers, symbols) = parse_input(input);
    // Filter all numbers that are near any symbol locations
    let ans: usize = numbers
        .iter()
        .filter_map(|n| {
            // if number is a part number return Some(number)
            if n.is_part_number(&symbols) {
                Some(n.num)
            } else {
                None
            }
        })
        .sum();
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_day3_part1() {
        let ans = day3_part1_solution(INPUT_STR.into());
        assert_eq!(ans, "4361");
    }
}
