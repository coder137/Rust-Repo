use std::path::PathBuf;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod day10;
mod day11;
mod day12;

fn main() {
    // NOTE, Just update this vector
    let solutions = [
        day1::day1_part1_solution,
        day1::day1_part2_solution,
        day2::day2_part1_solution,
        day2::day2_part2_solution,
        day3::day3_part1_solution,
        day3::day3_part2_solution,
        day4::day4_part1_solution,
        day4::day4_part2_solution,
        day5::day5_part1_solution,
        day5::day5_part2_solution,
        day6::day6_part1_solution,
        day6::day6_part2_solution,
        day7::day7_part1_solution,
        day7::day7_part2_solution,
        day8::day8_part1_solution,
        day8::day8_part2_solution,
        day9::day9_part1_solution,
        day9::day9_part2_solution,
        day10::day10_part1_solution,
        day10::day10_part2_solution,
        day11::day11_part1_solution,
        day11::day11_part2_solution,
        day12::day12_part1_solution,
        day12::day12_part2_solution,
    ];

    solutions.iter().enumerate().for_each(|(index, soln_cb)| {
        let day = index / 2 + 1;
        let part = index % 2 + 1;
        let solution = soln_cb(
            &PathBuf::new()
                .join("inputs")
                .join(format!("day{}_input.txt", day)),
        );
        println!("Day {} \tPart {} \t-> Solution: {}", day, part, solution);
    });
}
