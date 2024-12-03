use std::path::PathBuf;

mod day1;
mod day2;

fn main() {
    // NOTE, Just update this vector
    let solutions = [
        day1::day1_part1_solution,
        day1::day1_part2_solution,
        day2::day2_part1_solution,
        day2::day2_part2_solution,
    ];

    solutions.iter().enumerate().for_each(|(index, soln_cb)| {
        let day = index / 2 + 1;
        let part = index % 2 + 1;
        let input_file = PathBuf::new()
            .join("inputs")
            .join(format!("day{}_input.txt", day));
        let solution = soln_cb(common::read_file(&input_file));
        println!("Day {} \tPart {} \t-> Solution: {}", day, part, solution);
    });
}
