use std::{collections::HashSet, path::PathBuf};

fn is_lowest(data: &Vec<Vec<u8>>, point: (usize, usize)) -> bool {
    let x_max_length = data.len();
    let y_max_length = data[0].len();
    let (x_point, y_point) = point;
    let current_data = data[x_point][y_point];

    let mut is_lowest = true;
    if x_point.checked_sub(1) != None {
        is_lowest = is_lowest && data[x_point - 1][y_point] > current_data;
    }

    // NOTE, Can create a overflow_add(x_point, 1, max_length) -> Option<usize>
    // For now this is fine
    if x_point.checked_add(1) < Some(x_max_length) {
        is_lowest = is_lowest && data[x_point + 1][y_point] > current_data;
    }

    if y_point.checked_sub(1) != None {
        is_lowest = is_lowest && data[x_point][y_point - 1] > current_data;
    }

    if y_point.checked_add(1) < Some(y_max_length) {
        is_lowest = is_lowest && data[x_point][y_point + 1] > current_data;
    }
    is_lowest
}

fn get_lowest_points(data: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut lowest_points = Vec::new();
    data.iter().enumerate().for_each(|(x_counter, x_data)| {
        x_data.iter().enumerate().for_each(|(y_counter, _)| {
            // Current: x_counter, y_counter
            // Check (x_counter -1, y_counter), (x_counter + 1, y_counter)
            // Check (x_counter, y_counter - 1), (x_counter, y_counter + 1)
            let is_lowest = is_lowest(data, (x_counter, y_counter));
            if is_lowest {
                lowest_points.push((x_counter, y_counter));
            }
        })
    });
    lowest_points
}

fn day9_part1(data: &Vec<Vec<u8>>) -> u32 {
    get_lowest_points(data)
        .iter()
        .map(|p| data[p.0][p.1] as u32 + 1)
        .sum()
}

fn parse_values_from_file(path: &PathBuf) -> Vec<Vec<u8>> {
    let data = common::read_file(path);

    data.trim()
        .split("\n")
        .map(|x| {
            x.trim()
                .as_bytes()
                .iter()
                .map(|y| y - 0x30)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

pub fn day9_part1_solution(path: &PathBuf) -> String {
    let parsed_data = parse_values_from_file(path);
    day9_part1(&parsed_data).to_string()
}

struct Basin<'a> {
    data: &'a Vec<Vec<u8>>,
    smallest_point: (usize, usize),
}

impl<'a> Basin<'a> {
    fn new(data: &'a Vec<Vec<u8>>, smallest_point: (usize, usize)) -> Self {
        Basin {
            data,
            smallest_point,
        }
    }

    // Get all points in the basin (from the smallest point)
    fn get_all_points(&self) -> HashSet<(usize, usize)> {
        let mut verify_points = HashSet::new();
        verify_points.insert(self.smallest_point);

        let mut checked_points = Vec::new();
        checked_points.push(self.smallest_point);

        while !checked_points.is_empty() {
            let point = checked_points.pop().unwrap();
            let mut nearest_points: Vec<(usize, usize)> = self
                .get_nearest_points(point)
                .iter()
                .filter(|p| !verify_points.contains(p))
                .map(|&p| p)
                .collect();

            for np in &nearest_points {
                verify_points.insert(np.clone());
            }
            checked_points.append(&mut nearest_points);
        }

        verify_points
    }

    /// Get the nearest point (top, down, left, right) from the given point
    /// As long as the nearest point is a valid point and not part of the basin (9)
    fn get_nearest_points(&self, from_point: (usize, usize)) -> Vec<(usize, usize)> {
        let x_len = self.data.len() - 1;
        let y_len = self.data[0].len() - 1;

        let x = from_point.0;
        let y = from_point.1;

        // up, down, left, right
        let mut points = Vec::new();

        // left
        if x.checked_sub(1) != None {
            // self.data[x - 1][y];
            points.push((x - 1, y));
        }

        // right
        if x.checked_add(1) <= Some(x_len) {
            // self.data[x + 1][y];
            points.push((x + 1, y));
        }

        // down
        if y.checked_sub(1) != None {
            // self.data[x][y - 1];
            points.push((x, y - 1));
        }

        // up
        if y.checked_add(1) <= Some(y_len) {
            // self.data[x][y + 1];
            points.push((x, y + 1));
        }
        // println!("Points: {:?}", points);
        let points: Vec<(usize, usize)> = points
            .iter()
            .filter(|&&p| self.data[p.0][p.1] != 9)
            .map(|p| *p)
            .collect();

        points
    }
}

fn day9_part2(data: &Vec<Vec<u8>>) -> usize {
    let mut set = Vec::new();

    let lowest_points = get_lowest_points(data);
    for lp in lowest_points {
        let len = Basin::new(data, lp).get_all_points().len();
        set.push(len);
    }

    set.sort();

    set.iter()
        .rev()
        .take(3)
        .map(|x| *x)
        .reduce(|accum, item| (accum * item))
        .unwrap()
}

pub fn day9_part2_solution(path: &PathBuf) -> String {
    let parsed_data = parse_values_from_file(path);
    day9_part2(&parsed_data).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_part1_test() {
        let map = [
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];

        let parsed_data = map
            .iter()
            .map(|x| x.as_bytes().iter().map(|y| y - 0x30).collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();
        // println!("Parsed Data: {:?}", parsed_data);

        assert_eq!(day9_part1(&parsed_data), 15);
    }

    #[test]
    fn day9_part2_test() {
        let map = [
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];

        let parsed_data = map
            .iter()
            .map(|x| x.as_bytes().iter().map(|y| y - 0x30).collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();
        // println!("Parsed Data: {:?}", parsed_data);
        assert_eq!(parsed_data[0][1], 1);
        assert_eq!(parsed_data[0][9], 0);

        let basin = Basin::new(&parsed_data, (0, 9));
        assert_eq!(basin.get_nearest_points((0, 1)), vec![(0, 0)]);
        assert_eq!(basin.get_nearest_points((0, 9)), vec![(1, 9), (0, 8)]);
        assert_eq!(
            basin.get_all_points(),
            HashSet::from([
                (0, 9),
                (0, 8),
                (0, 7),
                (0, 6),
                (0, 5),
                (1, 6),
                (1, 8),
                (1, 9),
                (2, 9),
            ])
        );

        assert_eq!(day9_part2(&parsed_data), 1134);
    }
}
