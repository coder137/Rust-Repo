use std::collections::HashSet;

#[derive(Debug)]
enum GuardAlignment {
    Up,
    Down,
    Left,
    Right,
}

struct Map {
    grid_vertical_length: usize,
    grid_horizontal_length: usize,
    obstacles: HashSet<(usize, usize)>,
    guard_position: (usize, usize),
    guard_alignment: GuardAlignment,

    // state
    traversed: usize,
}

impl Map {
    fn try_up(&mut self) -> Option<()> {
        let xdir = self.guard_position.0;
        let next_xdir = match xdir.checked_sub(1) {
            Some(next_xdir) => next_xdir,
            None => return None,
        };

        // Check if there is an obstacle
        if self.obstacles.contains(&(next_xdir, self.guard_position.1)) {
            self.guard_alignment = GuardAlignment::Right;
        } else {
            // Move
            self.guard_position.0 = next_xdir;
            self.traversed += 1;
        }

        Some(())
    }

    fn try_down(&mut self) -> Option<()> {
        let xdir = self.guard_position.0;
        let next_xdir = xdir + 1;
        if next_xdir >= self.grid_vertical_length {
            return None;
        }

        // Check if there is an obstacle
        if self.obstacles.contains(&(next_xdir, self.guard_position.1)) {
            self.guard_alignment = GuardAlignment::Left;
        } else {
            // Move
            self.guard_position.0 = next_xdir;
            self.traversed += 1;
        }

        Some(())
    }

    fn try_left(&mut self) -> Option<()> {
        let ydir = self.guard_position.1;
        let next_ydir = match ydir.checked_sub(1) {
            Some(next_ydir) => next_ydir,
            None => return None,
        };

        // Check if there is an obstacle
        if self.obstacles.contains(&(self.guard_position.0, next_ydir)) {
            self.guard_alignment = GuardAlignment::Up;
        } else {
            // Move
            self.guard_position.1 = next_ydir;
            self.traversed += 1;
        }

        Some(())
    }

    fn try_right(&mut self) -> Option<()> {
        let ydir = self.guard_position.1;
        let next_ydir = ydir + 1;
        if next_ydir >= self.grid_horizontal_length {
            return None;
        }

        if self.obstacles.contains(&(self.guard_position.0, next_ydir)) {
            self.guard_alignment = GuardAlignment::Down;
        } else {
            self.guard_position.1 = next_ydir;
            self.traversed += 1;
        }

        Some(())
    }
}

impl Iterator for Map {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        match self.guard_alignment {
            GuardAlignment::Up => self.try_up(),
            GuardAlignment::Down => self.try_down(),
            GuardAlignment::Left => self.try_left(),
            GuardAlignment::Right => self.try_right(),
        }
    }
}

fn parse_input(input: String) -> Map {
    let input = input
        .trim()
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let grid_vertical_length = input.len();
    let grid_horizontal_length = input[0].len();

    let mut guard_position = None;
    let mut guard_alignment = None;
    let mut obstacles = HashSet::new();
    input.into_iter().enumerate().for_each(|(x, line)| {
        //
        line.into_iter().enumerate().for_each(|(y, c)| {
            //
            match c {
                '^' => {
                    guard_position = Some((x, y));
                    guard_alignment = Some(GuardAlignment::Up)
                }
                'v' => {
                    guard_position = Some((x, y));
                    guard_alignment = Some(GuardAlignment::Down)
                }
                '<' => {
                    guard_position = Some((x, y));
                    guard_alignment = Some(GuardAlignment::Left)
                }
                '>' => {
                    guard_position = Some((x, y));
                    guard_alignment = Some(GuardAlignment::Right)
                }
                '#' => {
                    obstacles.insert((x, y));
                }
                _ => {}
            }
        })
    });

    Map {
        grid_vertical_length,
        grid_horizontal_length,
        obstacles,
        guard_position: guard_position.unwrap(),
        guard_alignment: guard_alignment.unwrap(),
        traversed: 0,
    }
}

pub fn day6_part1_solution(input: String) -> String {
    let mut input = parse_input(input);

    let mut pos = HashSet::from([input.guard_position]);
    while input.next().is_some() {
        pos.insert(input.guard_position);
    }

    let ans = pos.len();
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        let ans = day6_part1_solution(INPUT_STR.into());
        println!("Ans: {ans}");
        assert_eq!(ans, "41");
    }
}
