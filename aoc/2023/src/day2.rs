#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<(usize, usize, usize)>,
}

fn parse_input(input: String) -> Vec<Game> {
    input
        .trim()
        .split('\n')
        .map(|l| {
            let p1 = l.trim().split(':').collect::<Vec<&str>>();
            let id = p1[0].trim().split(' ').collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();

            let sets = p1[1]
                .trim()
                .split(';')
                .map(|d| {
                    let mut r = 0;
                    let mut g = 0;
                    let mut b = 0;
                    d.split(',').for_each(|c| {
                        let color = c.trim().split(' ').collect::<Vec<&str>>();
                        let num = color[0].parse::<usize>().unwrap();
                        match color[1] {
                            "red" => {
                                r += num;
                            }
                            "green" => {
                                g += num;
                            }
                            "blue" => {
                                b += num;
                            }
                            _ => unreachable!(),
                        }
                    });
                    (r, g, b)
                })
                .collect::<Vec<(usize, usize, usize)>>();

            //
            Game { id, sets }
        })
        .collect::<Vec<Game>>()
}

pub fn day2_part1_solution(input: String) -> String {
    let games = parse_input(input);
    let ans: usize = games
        .iter()
        .filter(|g| {
            let is_invalid = g.sets.iter().any(|&(r, g, b)| r > 12 || g > 13 || b > 14);
            !is_invalid
        })
        .map(|g| g.id)
        .sum();
    ans.to_string()
}

pub fn day2_part2_solution(input: String) -> String {
    let games = parse_input(input);
    let ans: usize = games
        .iter()
        .map(|g| {
            let r_max = g.sets.iter().map(|&(r, _, _)| r).max().unwrap();
            let g_max = g.sets.iter().map(|&(_, g, _)| g).max().unwrap();
            let b_max = g.sets.iter().map(|&(_, _, b)| b).max().unwrap();
            r_max * g_max * b_max
        })
        .sum();
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n";

    #[test]
    fn test_day2_part1() {
        // let games = parse_input(INPUT_STR.into());
        // println!("{games:?}");
        let ans = day2_part1_solution(INPUT_STR.into());
        assert_eq!(ans, "8");
    }

    #[test]
    fn test_day2_part2() {
        let ans = day2_part2_solution(INPUT_STR.into());
        assert_eq!(ans, "2286");
    }
}
