use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
struct Octopus {
    energy_level: u8,
    flashed: bool,
    flash_counter: u32,
}

impl Octopus {
    fn new(energy_level: u8) -> Self {
        Octopus {
            energy_level,
            flashed: false,
            flash_counter: 0,
        }
    }

    /// Increment the energy level only if the octopus hasn't already flashed
    fn increment_energy_level(&mut self) {
        if !self.flashed {
            self.energy_level += 1;
        }
    }

    /// Invoke this function if you are sure that this octopus can_flash
    fn flash(&mut self) {
        self.energy_level = 0;
        self.flashed = true;
    }

    /// Reset the flashed counter value
    /// Increment the flash counter to track how many times each octopus has flashed
    fn end_step(&mut self) {
        if self.flashed {
            self.flash_counter += 1;
            self.flashed = false;
        }
    }

    fn can_flash(&self) -> bool {
        if self.energy_level > 9 && !self.flashed {
            true
        } else {
            false
        }
    }
}

struct OctopusSimulation {
    octopuses: Vec<Vec<Octopus>>,
}

impl OctopusSimulation {
    fn new(octopuses: Vec<Vec<Octopus>>) -> Self {
        OctopusSimulation { octopuses }
    }

    fn print(&self) {
        for vec in &self.octopuses {
            for o in vec {
                print!("{}", o.energy_level);
            }
            println!("");
        }
        println!("###########");
    }

    fn num_octopus_flashes(&self) -> u32 {
        self.octopuses
            .iter()
            .flatten()
            .map(|o| o.flash_counter)
            .sum()
    }

    fn is_synchronized_flash(&self) -> bool {
        self.octopuses.iter().flatten().all(|o| o.energy_level == 0)
    }

    fn get_location_of_octopuses_that_can_flash(&self) -> Vec<(usize, usize)> {
        let mut location_of_octopuses_that_can_flash = Vec::new();
        self.octopuses
            .iter()
            .enumerate()
            .for_each(|(x_pos, vec_o)| {
                vec_o.iter().enumerate().for_each(|(y_pos, o)| {
                    if o.can_flash() {
                        location_of_octopuses_that_can_flash.push((x_pos, y_pos));
                    }
                })
            });
        location_of_octopuses_that_can_flash
    }

    /// position_to_flash MUST be a flashable location
    fn compute_flash_for_an_octopus(&mut self, position_to_flash: (usize, usize)) {
        let x_pos = position_to_flash.0;
        let y_pos = position_to_flash.1;
        let x_len = self.octopuses.len() - 1;
        let y_len = self.octopuses[0].len() - 1;

        self.octopuses[x_pos][y_pos].flash();

        let can_x_up = x_pos + 1 <= x_len;
        let can_x_down = x_pos.checked_sub(1).is_some();

        let can_y_up = y_pos + 1 <= y_len;
        let can_y_down = y_pos.checked_sub(1).is_some();

        // up
        if can_x_up {
            // entity_flashes.push((x_pos + 1, y_pos));
            self.octopuses[x_pos + 1][y_pos].increment_energy_level();
        }

        // down
        if can_x_down {
            // entity_flashes.push((x_pos - 1, y_pos));
            self.octopuses[x_pos - 1][y_pos].increment_energy_level();
        }

        // right
        if can_y_up {
            // entity_flashes.push((x_pos, y_pos + 1));
            self.octopuses[x_pos][y_pos + 1].increment_energy_level();
        }

        // left
        if can_y_down {
            // entity_flashes.push((x_pos, y_pos - 1));
            self.octopuses[x_pos][y_pos - 1].increment_energy_level();
        }

        // up_right
        if can_x_up && can_y_up {
            // entity_flashes.push((x_pos + 1, y_pos + 1));
            self.octopuses[x_pos + 1][y_pos + 1].increment_energy_level();
        }

        // up_left
        if can_x_up && can_y_down {
            // entity_flashes.push((x_pos + 1, y_pos - 1));
            self.octopuses[x_pos + 1][y_pos - 1].increment_energy_level();
        }

        // down_right
        if can_x_down && can_y_up {
            // entity_flashes.push((x_pos - 1, y_pos + 1));
            self.octopuses[x_pos - 1][y_pos + 1].increment_energy_level();
        }

        // down_left
        if can_x_down && can_y_down {
            // entity_flashes.push((x_pos - 1, y_pos - 1));
            self.octopuses[x_pos - 1][y_pos - 1].increment_energy_level();
        }
    }
}

impl Iterator for OctopusSimulation {
    type Item = Vec<Vec<Octopus>>;

    fn next(&mut self) -> Option<Self::Item> {
        // 1. Increase energy level by 1
        self.octopuses
            .iter_mut()
            .for_each(|vec_e| vec_e.iter_mut().for_each(|e| e.increment_energy_level()));

        // 2. Flash all octupus with energy level above 9
        let mut flash_locations = self.get_location_of_octopuses_that_can_flash();
        while !flash_locations.is_empty() {
            for pos in flash_locations {
                self.compute_flash_for_an_octopus(pos.clone());
            }
            // println!("####################################################");
            // println!("Parsed Data: {iteration} {:#?}", self.octopuses);
            flash_locations = self.get_location_of_octopuses_that_can_flash();
        }

        // 3. Reset
        for vec in &mut self.octopuses {
            for o in vec {
                o.end_step();
            }
        }

        // 4. Return iteration
        Some(self.octopuses.clone())
    }
}

fn day11_part1(data: Vec<Vec<Octopus>>) -> u32 {
    let mut octopus_simulation = OctopusSimulation::new(data);
    octopus_simulation.nth(99).unwrap();
    octopus_simulation.num_octopus_flashes()
}

fn parse_values_from_file(path: &PathBuf) -> Vec<Vec<Octopus>> {
    common::read_file(path)
        .trim()
        .split("\n")
        .map(|x| {
            x.trim()
                .as_bytes()
                .iter()
                .map(|c| Octopus::new(c - 0x30))
                .collect::<Vec<Octopus>>()
        })
        .collect::<Vec<Vec<Octopus>>>()
}

pub fn day11_part1_solution(path: &PathBuf) -> String {
    let data = parse_values_from_file(path);
    day11_part1(data).to_string()
}

fn day11_part2(data: Vec<Vec<Octopus>>) -> u32 {
    let mut octopus_simulation = OctopusSimulation::new(data);
    let mut counter = 0;
    while !octopus_simulation.is_synchronized_flash() {
        octopus_simulation.next();
        counter += 1;
    }
    counter
}

pub fn day11_part2_solution(path: &PathBuf) -> String {
    let data = parse_values_from_file(path);
    day11_part2(data).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_part1_simple_simulation() {
        let data = ["11111", "19991", "19191", "19991", "11111"];

        let parsed_data = data
            .iter()
            .map(|x| {
                x.as_bytes()
                    .iter()
                    .map(|c| Octopus::new(c - 0x30))
                    .collect::<Vec<Octopus>>()
            })
            .collect::<Vec<Vec<Octopus>>>();

        {
            let mut octopus_simulation = OctopusSimulation::new(parsed_data.clone());
            octopus_simulation.nth(0).unwrap();
            octopus_simulation.print();
        }

        {
            let mut octopus_simulation = OctopusSimulation::new(parsed_data.clone());
            octopus_simulation.nth(1).unwrap();
            octopus_simulation.print();
        }
    }

    #[test]
    fn test_day11_part1_example_simulation() {
        let data = [
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ];

        let parsed_data = data
            .iter()
            .map(|x| {
                x.as_bytes()
                    .iter()
                    .map(|c| Octopus::new(c - 0x30))
                    .collect::<Vec<Octopus>>()
            })
            .collect::<Vec<Vec<Octopus>>>();

        {
            let mut octopus_simulation = OctopusSimulation::new(parsed_data.clone());
            octopus_simulation.nth(9).unwrap();
            octopus_simulation.print();

            assert_eq!(octopus_simulation.num_octopus_flashes(), 204);
        }

        {
            let mut octopus_simulation = OctopusSimulation::new(parsed_data.clone());
            octopus_simulation.nth(99).unwrap();
            octopus_simulation.print();

            assert_eq!(octopus_simulation.num_octopus_flashes(), 1656);
        }

        {
            let solution =
                day11_part1_solution(&PathBuf::new().join("inputs").join("day11_input.txt"));
            println!("Day11 Part1: {solution}");
        }
    }

    #[test]
    fn test_day11_part2_example_simulation() {
        let data = [
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ];

        let parsed_data = data
            .iter()
            .map(|x| {
                x.as_bytes()
                    .iter()
                    .map(|c| Octopus::new(c - 0x30))
                    .collect::<Vec<Octopus>>()
            })
            .collect::<Vec<Vec<Octopus>>>();

        {
            let mut octopus_simulation = OctopusSimulation::new(parsed_data.clone());

            octopus_simulation.nth(194).unwrap();
            assert!(octopus_simulation.is_synchronized_flash());
        }

        {
            let mut octopus_simulation = OctopusSimulation::new(parsed_data.clone());
            let mut counter = 0;
            while !octopus_simulation.is_synchronized_flash() {
                octopus_simulation.next();
                counter += 1;
            }
            assert_eq!(counter, 195);
        }
        {
            let solution =
                day11_part2_solution(&PathBuf::new().join("inputs").join("day11_input.txt"));
            println!("Day11 Part2: {solution}");
        }
    }
}
