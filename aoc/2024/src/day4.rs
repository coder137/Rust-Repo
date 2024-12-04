pub fn parse_input(input: String) -> Vec<Vec<char>> {
    input
        .trim()
        .split("\n")
        .map(|xline| xline.trim().chars().collect::<Vec<_>>())
        .collect()
}

const XMAS: &'static [char] = &['X', 'M', 'A', 'S'];

struct XmasGrid {
    grid: Vec<Vec<char>>,
    vertical_len: usize,
    horizontal_len: usize,
    xmas: &'static [char],
}

impl XmasGrid {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let vertical_len = grid.len();
        let horizontal_len = grid[0].len();
        Self {
            grid,
            vertical_len,
            horizontal_len,
            xmas: XMAS,
        }
    }

    pub fn compute_all_directions(&self, point: (usize, usize)) -> u32 {
        let mut count = 0;
        count += self.east(point) as u32;
        count += self.west(point) as u32;
        count += self.north(point) as u32;
        count += self.south(point) as u32;
        count += self.northeast(point) as u32;
        count += self.northwest(point) as u32;
        count += self.southeast(point) as u32;
        count += self.southwest(point) as u32;
        count
    }

    // XMAS
    // east
    fn east(&self, point: (usize, usize)) -> bool {
        let ymax = point.1 + 3;
        if ymax >= self.horizontal_len {
            return false;
        }
        let c = &self.grid[point.0][point.1..=ymax];
        c == self.xmas
    }

    // SAMX
    // west
    fn west(&self, point: (usize, usize)) -> bool {
        let ymin = point.1.checked_sub(3);
        let ymin = match ymin {
            Some(ymin) => ymin,
            None => return false,
        };

        let c = &self.grid[point.0][ymin..=point.1];
        self.xmas.iter().rev().eq(c.iter())
    }

    // X
    // M
    // A
    // S
    fn south(&self, point: (usize, usize)) -> bool {
        let xmax = point.0 + 3;
        if xmax >= self.vertical_len {
            return false;
        }
        let mut c = ['0'; 4];
        let mut count = 0;
        for x in point.0..=xmax {
            c[count] = self.grid[x][point.1];
            count += 1;
        }
        c == self.xmas
    }

    // S
    // A
    // M
    // X
    fn north(&self, point: (usize, usize)) -> bool {
        let xmin = point.0.checked_sub(3);
        let xmin = match xmin {
            Some(xmin) => xmin,
            None => return false,
        };
        let mut c = ['0'; 4];
        let mut count = 0;
        for x in (xmin..=point.0).rev() {
            c[count] = self.grid[x][point.1];
            count += 1;
        }
        c == self.xmas
    }

    //    S
    //   A
    //  M
    // X
    fn northeast(&self, point: (usize, usize)) -> bool {
        let xmin = point.0.checked_sub(3);
        let xmin = match xmin {
            Some(xmin) => xmin,
            None => return false,
        };
        let ymax = point.1 + 3;
        if ymax >= self.horizontal_len {
            return false;
        }

        let mut c = ['0'; 4];
        let mut counter = 0;
        for (x, y) in (xmin..=point.0).rev().zip(point.1..=ymax) {
            c[counter] = self.grid[x][y];
            counter += 1;
        }
        c == self.xmas
    }

    // S
    //  A
    //   M
    //    X
    fn northwest(&self, point: (usize, usize)) -> bool {
        let xmin = point.0.checked_sub(3);
        let xmin = match xmin {
            Some(xmin) => xmin,
            None => return false,
        };
        let ymin = point.1.checked_sub(3);
        let ymin = match ymin {
            Some(ymin) => ymin,
            None => return false,
        };
        let mut c = ['0'; 4];
        let mut counter = 0;
        for (x, y) in (xmin..=point.0).rev().zip((ymin..=point.1).rev()) {
            c[counter] = self.grid[x][y];
            counter += 1;
        }
        c == self.xmas
    }

    // X
    //  M
    //   A
    //    S
    fn southeast(&self, point: (usize, usize)) -> bool {
        let xmax = point.0 + 3;
        if xmax >= self.vertical_len {
            return false;
        }
        let ymax = point.1 + 3;
        if ymax >= self.horizontal_len {
            return false;
        }

        let mut c = ['0'; 4];
        let mut counter = 0;

        for (x, y) in (point.0..=xmax).zip(point.1..=ymax) {
            c[counter] = self.grid[x][y];
            counter += 1;
        }
        c == self.xmas
    }

    //    X
    //   M
    //  A
    // S
    fn southwest(&self, point: (usize, usize)) -> bool {
        let xmax = point.0 + 3;
        if xmax >= self.vertical_len {
            return false;
        }
        let ymin = point.1.checked_sub(3);
        let ymin = match ymin {
            Some(ymin) => ymin,
            None => return false,
        };

        let mut c = ['0'; 4];
        let mut counter = 0;

        for (x, y) in (point.0..=xmax).zip((ymin..=point.1).rev()) {
            c[counter] = self.grid[x][y];
            counter += 1;
        }
        c == self.xmas
    }
}

pub fn day4_part1_solution(input: String) -> String {
    let input = parse_input(input);
    let xlist = input
        .iter()
        .enumerate()
        .map(|(x, xline)| {
            //
            xline.into_iter().enumerate().filter_map(move |(y, ychar)| {
                //
                if *ychar == 'X' {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    let grid = XmasGrid::new(input);
    let ans = xlist
        .into_iter()
        .map(|point| grid.compute_all_directions(point))
        .sum::<u32>();
    ans.to_string()
}

struct XShapedMasGrid {
    grid: Vec<Vec<char>>,
    vertical_len: usize,
    horizontal_len: usize,
}

impl XShapedMasGrid {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let vertical_len = grid.len();
        let horizontal_len = grid[0].len();
        Self {
            grid,
            vertical_len,
            horizontal_len,
        }
    }

    // M . M
    // . A .
    // S . S
    // point is coordinates of A
    pub fn is_x_mas(&self, point: (usize, usize)) -> bool {
        let xmax = point.0 + 1;
        if xmax >= self.vertical_len {
            return false;
        }
        let xmin = point.0.checked_sub(1);
        let xmin = match xmin {
            Some(xmin) => xmin,
            None => return false,
        };
        let ymax = point.1 + 1;
        if ymax >= self.horizontal_len {
            return false;
        }
        let ymin = point.1.checked_sub(1);
        let ymin = match ymin {
            Some(ymin) => ymin,
            None => return false,
        };

        let top_left_to_bottom_right = {
            // cross from top left to bottom right
            let mut c = ['0'; 3];
            let mut counter = 0;
            for (x, y) in (xmin..=xmax).zip(ymin..=ymax) {
                c[counter] = self.grid[x][y];
                counter += 1;
            }

            c == ['M', 'A', 'S'] || c == ['S', 'A', 'M']
        };

        let bottom_left_to_top_right = {
            // cross from bottom left to top right
            let mut c = ['0'; 3];
            let mut counter = 0;
            for (x, y) in (xmin..=xmax).rev().zip(ymin..=ymax) {
                c[counter] = self.grid[x][y];
                counter += 1;
            }

            c == ['M', 'A', 'S'] || c == ['S', 'A', 'M']
        };

        top_left_to_bottom_right && bottom_left_to_top_right
    }
}

pub fn day4_part2_solution(input: String) -> String {
    let input = parse_input(input);
    let alist = input
        .iter()
        .enumerate()
        .map(|(x, xline)| {
            //
            xline.into_iter().enumerate().filter_map(move |(y, ychar)| {
                //
                if *ychar == 'A' {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    let grid = XShapedMasGrid::new(input);
    let ans = alist
        .into_iter()
        .map(|apoint| {
            //
            grid.is_x_mas(apoint) as u32
        })
        .sum::<u32>();
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        let ans = day4_part1_solution(INPUT_STR.into());
        println!("Ans: {ans}");
        assert_eq!(ans, "18");
    }

    #[test]
    fn test_part2() {
        let ans = day4_part2_solution(INPUT_STR.into());
        println!("Ans: {ans}");
        assert_eq!(ans, "9");
    }
}
