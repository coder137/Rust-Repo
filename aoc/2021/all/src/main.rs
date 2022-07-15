use day1;
use day2;
use day3;
use day4;
use day5;
use day6;

fn main() {
    // NOTE, Just update this vector
    let solutions = [
        day1::day1_part1_solution(),
        day1::day1_part2_solution(),
        day2::day2_part1_solution(),
        day2::day2_part2_solution(),
        day3::day3_part1_solution(),
        day3::day3_part2_solution(),
        day4::day4_part1_solution(),
        day4::day4_part2_solution(),
        day5::day5_part1_solution(),
        day5::day5_part2_solution(),
        day6::day6_part1_solution(),
        day6::day6_part2_solution(),
    ];

    solutions.iter().enumerate().for_each(|(index, data)| {
        println!(
            "Day {} Part {} -> Solution: {}",
            index / 2 + 1,
            index % 2 + 1,
            data
        );
    });
}
