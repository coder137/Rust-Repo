use std::collections::HashMap;

use day1;

fn main() {
    let solutions: HashMap<u8, String> = HashMap::from([
        (0, day1::day1_part1_solution()),
        (1, day1::day1_part2_solution()),
    ]);

    let length = solutions.len() as u8;
    (0..length).for_each(|x| {
        println!(
            "Day {} Part {} -> Solution: {}",
            x / 2,
            x % 2,
            solutions[&x]
        );
    });
}
