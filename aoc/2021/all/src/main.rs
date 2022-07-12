use day1;

fn main() {
    let solutions = vec![day1::day1_part1_solution(), day1::day1_part2_solution()];

    solutions.iter().enumerate().for_each(|(index, data)| {
        println!(
            "Day {} Part {} -> Solution: {}",
            index / 2 + 1,
            index % 2 + 1,
            data
        );
    })
}
