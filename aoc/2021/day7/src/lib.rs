use std::path::PathBuf;

fn get_max(data: &[u32]) -> u32 {
    *data.iter().max().unwrap()
}

fn get_min(data: &[u32]) -> u32 {
    *data.iter().min().unwrap()
}

fn compute_fuel_usage(data: &[u32], pos: u32) -> u32 {
    data.iter().map(|x| x.abs_diff(pos)).sum()
}

// NOTE, Incomplete implementation (no checks or guards in place)
fn day7_part1(data: &[u32]) -> u32 {
    let low = get_min(data);
    let high = get_max(data);

    let mut current_low = low;
    let mut current_high = high;
    let mut current_mid = current_low + (current_high - current_low) / 2;

    // Compute the mid value (+1 and -1 value)
    let mut final_answer = 0;
    while low <= high {
        let abs_mid = compute_fuel_usage(data, current_mid);
        let abs_low = compute_fuel_usage(data, current_mid - 1);
        let abs_high = compute_fuel_usage(data, current_mid + 1);
        // println!("Mid: {} {}", abs_mid, current_mid);
        if abs_mid < abs_low && abs_mid < abs_high {
            // This is the correct answer
            // println!("Correct answer: {} {}", abs_mid, current_mid);
            final_answer = abs_mid;
            break;
        } else if abs_low < abs_mid {
            // Value is lower than current_mid
            current_high = current_mid;
        } else {
            // Value is higher than current_mid
            current_low = current_mid;
        }
        current_mid = current_low + (current_high - current_low) / 2;
    }
    final_answer
}

pub fn day7_part1_solution() -> String {
    let data = common::read_file(&PathBuf::new().join("day7").join("input.txt"));

    let parsed_data = data
        .trim()
        .split(',')
        .map(|x| u32::from_str_radix(x, 10).unwrap())
        .collect::<Vec<u32>>();
    day7_part1(&parsed_data).to_string()
}

fn compute_fuel_usage_increased_burn(data: &[u32], pos: u32) -> u32 {
    data.iter()
        .map(|x| {
            let diff = x.abs_diff(pos);
            // Sum of numbers from (1..N)
            diff * (diff + 1) / 2
        })
        .sum()
}

fn day7_part2(data: &[u32]) -> u32 {
    let low = get_min(data);
    let high = get_max(data);

    let mut current_low = low;
    let mut current_high = high;
    let mut current_mid = current_low + (current_high - current_low) / 2;

    // Compute the mid value (+1 and -1 value)
    let mut final_answer = 0;
    while low <= high {
        let abs_mid = compute_fuel_usage_increased_burn(data, current_mid);
        let abs_low = compute_fuel_usage_increased_burn(data, current_mid - 1);
        let abs_high = compute_fuel_usage_increased_burn(data, current_mid + 1);
        // println!("Mid: {} {}", abs_mid, current_mid);
        if abs_mid < abs_low && abs_mid < abs_high {
            // This is the correct answer
            // println!("Correct answer: {} {}", abs_mid, current_mid);
            final_answer = abs_mid;
            break;
        } else if abs_low < abs_mid {
            // Value is lower than current_mid
            current_high = current_mid;
        } else {
            // Value is higher than current_mid
            current_low = current_mid;
        }
        current_mid = current_low + (current_high - current_low) / 2;
    }
    final_answer
}

pub fn day7_part2_solution() -> String {
    let data = common::read_file(&PathBuf::new().join("day7").join("input.txt"));

    let parsed_data = data
        .trim()
        .split(',')
        .map(|x| u32::from_str_radix(x, 10).unwrap())
        .collect::<Vec<u32>>();
    day7_part2(&parsed_data).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_part1_test() {
        let data = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(*data.iter().max().unwrap(), 16);
        assert_eq!(*data.iter().min().unwrap(), 0);

        assert_eq!(compute_fuel_usage(&data, 2), 37);
        assert_eq!(compute_fuel_usage(&data, 1), 41);
        assert_eq!(compute_fuel_usage(&data, 3), 39);

        assert_eq!(day7_part1(&data), 37);
    }

    #[test]
    fn day7_part2_test() {
        let data = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(compute_fuel_usage_increased_burn(&data, 5), 168);
        assert_eq!(compute_fuel_usage_increased_burn(&data, 2), 206);
        assert_eq!(day7_part2(&data), 168);
    }
}
