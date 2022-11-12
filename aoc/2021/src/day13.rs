use std::{collections::HashSet, path::PathBuf};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Instruction {
    index: usize,
    axis: String,
}

impl Instruction {
    fn new(index: usize, axis: String) -> Self {
        Self { index, axis }
    }
}

struct TransparentPaper {
    points: Vec<Point>,
}

impl TransparentPaper {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn count_points(&self) -> usize {
        let mut set = HashSet::new();
        for p in &self.points {
            set.insert((p.x, p.y));
        }
        set.len()
    }

    fn perform_instruction(&mut self, instruction: &Instruction) {
        if instruction.axis == "y" {
            self.points
                .iter_mut()
                .filter(|p| match p.y.checked_sub(instruction.index) {
                    Some(val) => val > 0,
                    None => false,
                })
                .for_each(|p| p.y = instruction.index - (p.y - instruction.index));
        }

        if instruction.axis == "x" {
            self.points
                .iter_mut()
                .filter(|p| match p.x.checked_sub(instruction.index) {
                    Some(val) => val > 0,
                    None => false,
                })
                .for_each(|p| p.x = instruction.index - (p.x - instruction.index));
        }
    }

    fn get_max_points(&self) -> Point {
        let max_x = self.points.iter().map(|p| p.x).max().unwrap();
        let max_y = self.points.iter().map(|p| p.y).max().unwrap();
        Point::new(max_x + 1, max_y + 1)
    }

    fn graph_as_string(&self) -> String {
        // Initialize
        let max_points = self.get_max_points();
        let mut final_vec = vec![vec!["."; max_points.x]; max_points.y];

        // Plot points
        self.points.iter().for_each(|p| final_vec[p.y][p.x] = "#");

        // Final output
        let final_str = final_vec
            .iter()
            .map(|l| l.join(""))
            .collect::<Vec<String>>();
        format!("{:#?}", final_str)
    }

    fn print_graph(&self) {
        println!("{}", self.graph_as_string());
    }
}

fn parse_values_from_file(path: &PathBuf) -> (Vec<Point>, Vec<Instruction>) {
    let split_string = common::read_file(path)
        .trim()
        .split("\n")
        .map(|s| s.trim().to_owned())
        .collect::<Vec<String>>();

    let mut points = Vec::new();
    let mut instruction_start_line = None;

    for (index, line) in split_string.iter().enumerate() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            instruction_start_line = Some(index);
            break;
        }

        let point_str = trimmed_line.split(",").collect::<Vec<&str>>();
        // println!("Point String: {:?}", point_str);
        let x = point_str[0].parse::<usize>().unwrap();
        let y = point_str[1].parse::<usize>().unwrap();
        points.push(Point::new(x, y));
    }

    let mut instructions = Vec::new();

    for line in split_string
        .iter()
        .skip(instruction_start_line.unwrap() + 1)
    {
        let instruction_str = line.split("fold along ").collect::<Vec<&str>>()[1]
            .split("=")
            .collect::<Vec<&str>>();

        let instruction = Instruction::new(
            instruction_str[1].parse::<usize>().unwrap(),
            instruction_str[0].to_owned(),
        );
        instructions.push(instruction);
    }

    (points, instructions)
}

fn day13_part1(points: Vec<Point>, instructions: Vec<Instruction>) -> usize {
    let mut transparent_paper = TransparentPaper::new(points);
    if instructions.len() > 0 {
        transparent_paper.perform_instruction(&instructions[0]);
    }
    // transparent_paper.print_graph();
    transparent_paper.count_points()
}

pub fn day13_part1_solution(path: &PathBuf) -> String {
    let (points, instructions) = parse_values_from_file(path);
    day13_part1(points, instructions).to_string()
}

fn day13_part2(points: Vec<Point>, instructions: Vec<Instruction>) -> String {
    let mut transparent_paper = TransparentPaper::new(points);
    for instruction in instructions {
        transparent_paper.perform_instruction(&instruction);
    }
    transparent_paper.graph_as_string()
}

pub fn day13_part2_solution(path: &PathBuf) -> String {
    let (points, instructions) = parse_values_from_file(path);
    day13_part2(points, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse() -> (Vec<Point>, Vec<Instruction>) {
        let data = "6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0
        
        fold along y=7
        fold along x=5";

        let split_string = data.split("\n").collect::<Vec<&str>>();

        let mut points = Vec::new();
        let mut instruction_start_line = None;

        for (index, line) in split_string.iter().enumerate() {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                instruction_start_line = Some(index);
                break;
            }

            let point_str = trimmed_line.split(",").collect::<Vec<&str>>();
            let x = point_str[0].parse::<usize>().unwrap();
            let y = point_str[1].parse::<usize>().unwrap();
            points.push(Point::new(x, y));
        }

        let mut instructions = Vec::new();

        for line in split_string
            .iter()
            .skip(instruction_start_line.unwrap() + 1)
        {
            let instruction_str = line.split("fold along ").collect::<Vec<&str>>()[1]
                .split("=")
                .collect::<Vec<&str>>();

            let instruction = Instruction::new(
                instruction_str[1].parse::<usize>().unwrap(),
                instruction_str[0].to_owned(),
            );
            instructions.push(instruction);
        }
        (points, instructions)
    }

    #[test]
    fn test_data() {
        let (points, instructions) = test_parse();
        let solution = day13_part1(points, instructions);
        assert_eq!(solution, 17);
    }

    #[test]
    fn test_day13_part1_solution() {
        let solution = day13_part1_solution(&PathBuf::new().join("inputs").join("day13_input.txt"));
        println!("Solution: {}", solution);
    }

    #[test]
    fn test_day13_part2_solution() {
        let (points, instructions) = test_parse();
        day13_part2(points, instructions);

        //cargo test --package all --bin all -- day13::tests::test_day13_part2_solution --exact --nocapture
        let solution = day13_part2_solution(&PathBuf::new().join("inputs").join("day13_input.txt"));
        println!("Solution: {}", solution);
    }
}
