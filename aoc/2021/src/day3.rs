use std::path::PathBuf;

// Helper function
fn sum_bits(data: &Vec<&str>) -> Vec<u32> {
    data.iter()
        .map(|x| {
            x.trim()
                .as_bytes()
                .iter()
                .map(|&x| (x - 0x30) as u32)
                .collect::<Vec<u32>>()
        })
        .reduce(|x, y| {
            x.iter()
                .zip(y.iter())
                .map(|(&x, &y)| x + y)
                .collect::<Vec<u32>>()
        })
        .unwrap()
}

// Helper function
fn sum_bit_at_position(data: &Vec<&str>, pos: usize) -> u32 {
    data.iter().map(|x| (x.as_bytes()[pos] - 0x30) as u32).sum()
}

fn day3_part1(data: &Vec<&str>) -> String {
    let length_compare = data.len() as f32 / 2.0;

    // Compute gamma and epsilon
    let sum_bits = sum_bits(data);
    let gamma = sum_bits
        .iter()
        .map(|&x| {
            if x as f32 >= length_compare {
                1_u32
            } else {
                0_u32
            }
        })
        .rev()
        .enumerate()
        .map(|(index, value)| (value * (1 << index)))
        .sum::<u32>();
    let epsilon = sum_bits
        .iter()
        .map(|&x| {
            if x as f32 >= length_compare {
                0_u32
            } else {
                1_u32
            }
        })
        .rev()
        .enumerate()
        .map(|(index, value)| (value * (1 << index)))
        .sum::<u32>();
    (gamma * epsilon).to_string()
}

pub fn day3_part1_solution(path: &PathBuf) -> String {
    let data = common::read_file(path);
    let split = data.trim().split("\n").collect::<Vec<&str>>();
    day3_part1(&split)
}

fn day3_part2(data: &Vec<&str>) -> String {
    let max_from_sum = |value, comp| {
        if value >= comp {
            1
        } else {
            0
        }
    };
    let min_from_sum = |value, comp| {
        if value >= comp {
            0
        } else {
            1
        }
    };

    let mut counter = 0;
    let mut sum_bit_at_pos = sum_bit_at_position(data, counter);

    let mut o2_filtered = data
        .iter()
        .filter(|&x| {
            (x.as_bytes()[counter] - 0x30) as u32
                == max_from_sum(sum_bit_at_pos as f32, data.len() as f32 / 2.0)
        })
        .map(|x| *x)
        .collect::<Vec<&str>>();

    let mut co2_filtered = data
        .iter()
        .filter(|&x| {
            (x.as_bytes()[counter] - 0x30) as u32
                == min_from_sum(sum_bit_at_pos as f32, data.len() as f32 / 2.0)
        })
        .map(|x| *x)
        .collect::<Vec<&str>>();

    while o2_filtered.len() != 1 && counter <= data.len() {
        counter += 1;
        sum_bit_at_pos = sum_bit_at_position(&o2_filtered, counter);
        o2_filtered = o2_filtered
            .iter()
            .filter(|&x| {
                (x.as_bytes()[counter] - 0x30) as u32
                    == max_from_sum(sum_bit_at_pos as f32, o2_filtered.len() as f32 / 2.0)
            })
            .map(|x| *x)
            .collect::<Vec<&str>>();
    }

    counter = 0;
    while co2_filtered.len() != 1 && counter <= data.len() {
        counter += 1;
        sum_bit_at_pos = sum_bit_at_position(&co2_filtered, counter);
        co2_filtered = co2_filtered
            .iter()
            .filter(|&x| {
                (x.as_bytes()[counter] - 0x30) as u32
                    == min_from_sum(sum_bit_at_pos as f32, co2_filtered.len() as f32 / 2.0)
            })
            .map(|x| *x)
            .collect::<Vec<&str>>();
    }

    let o2 = u32::from_str_radix(o2_filtered[0].trim(), 2).unwrap();
    let co2 = u32::from_str_radix(co2_filtered[0].trim(), 2).unwrap();

    (o2 * co2).to_string()
}

pub fn day3_part2_solution(path: &PathBuf) -> String {
    let data = common::read_file(path);
    let split = data.trim().split("\n").collect::<Vec<&str>>();
    day3_part2(&split)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3() {
        let ans = day3_part1(&vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]);
        assert_eq!(ans, "198");
    }

    #[test]
    fn test_day3_part2() {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let ans = day3_part2(&data);
        assert_eq!(ans, "230");
    }

    #[test]
    fn test_arr() {
        let arr = [1, 0, 1, 1, 1];
        let s = arr
            .iter()
            .map(|&x| char::from_digit(x, 10).unwrap())
            .collect::<String>();
        assert_eq!(s, "10111");
    }
}
