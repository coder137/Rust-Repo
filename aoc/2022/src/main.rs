use std::path::PathBuf;

mod day1;

fn main() {
    // NOTE, Just update this vector
    let solutions = [day1::day1_part1_solution];

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
