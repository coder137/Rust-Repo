use std::path::PathBuf;

#[derive(Debug, PartialEq, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

impl RPS {
    fn is_win(opponent: &RPS, player: &RPS) -> bool {
        (player == &RPS::Paper && opponent == &RPS::Rock)
            || (player == &RPS::Scissor && opponent == &RPS::Paper)
            || (player == &RPS::Rock && opponent == &RPS::Scissor)
    }

    fn is_draw(opponent: &RPS, player: &RPS) -> bool {
        opponent == player
    }

    fn is_lose(opponent: &RPS, player: &RPS) -> bool {
        !RPS::is_win(opponent, player) && !RPS::is_draw(opponent, player)
    }

    fn to_num(e: &RPS) -> u32 {
        match e {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissor => 3,
        }
    }
}

fn day2_part1(data: &Vec<(RPS, RPS)>) -> u32 {
    data.iter()
        .map(|(prediction, player_move)| {
            let player_move_score = RPS::to_num(player_move);
            if RPS::is_win(prediction, player_move) {
                6 + player_move_score
            } else if RPS::is_draw(prediction, player_move) {
                3 + player_move_score
            } else {
                0 + player_move_score
            }
        })
        .sum::<u32>()
}

fn parse_values_from_file(path: &PathBuf) -> Vec<(RPS, RPS)> {
    let input = common::read_file(path);
    input
        .trim()
        .split("\n")
        .map(|line| {
            let rps_pair = line.trim().split(" ").collect::<Vec<&str>>();
            let prediction = match rps_pair[0] {
                "A" => RPS::Rock,
                "B" => RPS::Paper,
                "C" => RPS::Scissor,
                _ => panic!("Invalid option"),
            };

            let player_move = match rps_pair[1] {
                "X" => RPS::Rock,
                "Y" => RPS::Paper,
                "Z" => RPS::Scissor,
                _ => panic!("Invalid option"),
            };

            (prediction, player_move)
        })
        .collect::<Vec<(RPS, RPS)>>()
}

pub fn day2_part1_solution(path: &PathBuf) -> String {
    let parsed_data = parse_values_from_file(path);
    day2_part1(&parsed_data).to_string()
}

enum Action {
    Lose,
    Draw,
    Win,
}

impl Action {
    fn for_lose(opponent: &RPS) -> RPS {
        match opponent {
            RPS::Rock => RPS::Scissor,
            RPS::Paper => RPS::Rock,
            RPS::Scissor => RPS::Paper,
        }
    }

    fn for_draw(opponent: &RPS) -> RPS {
        *opponent
    }

    fn for_win(opponent: &RPS) -> RPS {
        match opponent {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissor,
            RPS::Scissor => RPS::Rock,
        }
    }
}

fn day2_part2(data: &Vec<(RPS, Action)>) -> u32 {
    data.iter()
        .map(|(opponent, player_action)| match player_action {
            Action::Lose => 0 + RPS::to_num(&Action::for_lose(opponent)),
            Action::Draw => 3 + RPS::to_num(&Action::for_draw(opponent)),
            Action::Win => 6 + RPS::to_num(&Action::for_win(opponent)),
        })
        .sum()
}

fn parse_values_from_file_for_part2(path: &PathBuf) -> Vec<(RPS, Action)> {
    let input = common::read_file(path);
    input
        .trim()
        .split("\n")
        .map(|line| {
            let rps_pair = line.trim().split(" ").collect::<Vec<&str>>();
            let prediction = match rps_pair[0] {
                "A" => RPS::Rock,
                "B" => RPS::Paper,
                "C" => RPS::Scissor,
                _ => panic!("Invalid option"),
            };

            let player_move = match rps_pair[1] {
                "X" => Action::Lose,
                "Y" => Action::Draw,
                "Z" => Action::Win,
                _ => panic!("Invalid option"),
            };

            (prediction, player_move)
        })
        .collect::<Vec<(RPS, Action)>>()
}

pub fn day2_part2_solution(path: &PathBuf) -> String {
    let parsed_data = parse_values_from_file_for_part2(path);
    day2_part2(&parsed_data).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input_parse() -> Vec<(RPS, RPS)> {
        let input = "A Y
        B X
        C Z";

        input
            .trim()
            .split("\n")
            .map(|line| {
                let rps_pair = line.trim().split(" ").collect::<Vec<&str>>();
                let prediction = match rps_pair[0] {
                    "A" => RPS::Rock,
                    "B" => RPS::Paper,
                    "C" => RPS::Scissor,
                    _ => panic!("Invalid option"),
                };

                let player_move = match rps_pair[1] {
                    "X" => RPS::Rock,
                    "Y" => RPS::Paper,
                    "Z" => RPS::Scissor,
                    _ => panic!("Invalid option"),
                };

                (prediction, player_move)
            })
            .collect::<Vec<(RPS, RPS)>>()
    }

    fn test_input_parse_for_p2() -> Vec<(RPS, Action)> {
        let input = "A Y
        B X
        C Z";

        input
            .trim()
            .split("\n")
            .map(|line| {
                let rps_pair = line.trim().split(" ").collect::<Vec<&str>>();
                let prediction = match rps_pair[0] {
                    "A" => RPS::Rock,
                    "B" => RPS::Paper,
                    "C" => RPS::Scissor,
                    _ => panic!("Invalid option"),
                };

                let player_move = match rps_pair[1] {
                    "X" => Action::Lose,
                    "Y" => Action::Draw,
                    "Z" => Action::Win,
                    _ => panic!("Invalid option"),
                };

                (prediction, player_move)
            })
            .collect::<Vec<(RPS, Action)>>()
    }

    #[test]
    fn test_day2_part1() {
        let parsed_data = test_input_parse();
        println!("Parsed Data: {:?}", parsed_data);

        assert!(RPS::is_draw(&RPS::Rock, &RPS::Rock));
        assert!(RPS::is_draw(&RPS::Paper, &RPS::Paper));
        assert!(RPS::is_draw(&RPS::Scissor, &RPS::Scissor));

        let final_answer = parsed_data
            .iter()
            .map(|(prediction, player_move)| {
                if RPS::is_win(prediction, player_move) {
                    6 + RPS::to_num(player_move)
                } else if RPS::is_draw(prediction, player_move) {
                    3 + RPS::to_num(player_move)
                } else {
                    0 + RPS::to_num(player_move)
                }
            })
            .sum::<u32>();
        assert_eq!(final_answer, 15);

        assert_eq!(day2_part1(&parsed_data), 15);
    }

    #[test]
    fn test_day2_part2() {
        let parsed_data = test_input_parse_for_p2();
        assert_eq!(day2_part2(&parsed_data), 12);
    }
}
