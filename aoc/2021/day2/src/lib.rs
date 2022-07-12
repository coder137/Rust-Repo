use std::path::PathBuf;

#[derive(Debug)]
struct Value {
    length: u32,
    depth: u32,
}

impl Value {
    fn new() -> Self {
        Value {
            length: 0,
            depth: 0,
        }
    }

    fn product(&self) -> u32 {
        self.length * self.depth
    }

    fn add_length(&self, len: u32) -> Self {
        Value {
            length: self.length + len,
            depth: self.depth,
        }
    }

    fn add_depth(&self, depth: u32) -> Self {
        Value {
            length: self.length,
            depth: self.depth + depth,
        }
    }

    fn remove_depth(&self, depth: u32) -> Self {
        Value {
            length: self.length,
            depth: self.depth - depth,
        }
    }
}

fn day2_part1(data: &Vec<&str>) -> String {
    let fvalue = data.iter().fold(Value::new(), |acc, x| {
        let splitstr: Vec<&str> = x.trim().split(" ").collect();
        let identifier = splitstr[0];
        let value = splitstr[1].parse::<u32>().unwrap();

        // Return one of these values
        match identifier {
            "forward" => acc.add_length(value),
            "down" => acc.add_depth(value),
            "up" => acc.remove_depth(value),
            _ => Value::new(),
        }
    });
    fvalue.product().to_string()
}

pub fn day2_part1_solution() -> String {
    let data = common::read_file(&PathBuf::new().join("day2").join("input.txt"));
    let split: Vec<&str> = data.trim().split("\n").collect();
    day2_part1(&split)
}

//

#[derive(Debug)]
struct ValueWithAim {
    value: Value,
    aim: u32,
}

impl ValueWithAim {
    fn new() -> Self {
        ValueWithAim {
            value: Value::new(),
            aim: 0,
        }
    }
}

fn day2_part2(data: &Vec<&str>) -> String {
    let fvalue = data.iter().fold(ValueWithAim::new(), |acc, x| {
        let splitstr: Vec<&str> = x.trim().split(" ").collect();
        let identifier = splitstr[0];
        let value = splitstr[1].parse::<u32>().unwrap();

        // Return one of these values
        match identifier {
            "forward" => ValueWithAim {
                value: Value {
                    length: acc.value.length + value,
                    depth: acc.value.depth + acc.aim * value,
                },
                aim: acc.aim,
            },
            "down" => ValueWithAim {
                value: acc.value,
                aim: acc.aim + value,
            },

            "up" => ValueWithAim {
                value: acc.value,
                aim: acc.aim - value,
            },
            _ => ValueWithAim::new(),
        }
    });
    fvalue.value.product().to_string()
}

pub fn day2_part2_solution() -> String {
    let data = common::read_file(&PathBuf::new().join("day2").join("input.txt"));
    let split: Vec<&str> = data.trim().split("\n").collect();
    day2_part2(&split)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() {
        let counter = day2_part1(&vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]);
        assert_eq!(counter, "150");
    }

    #[test]
    fn test_day2_part2() {
        let counter = day2_part2(&vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]);
        assert_eq!(counter, "900");
    }
}
