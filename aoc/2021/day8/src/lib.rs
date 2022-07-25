struct Pattern<'a> {
    unique_pattern: Vec<&'a str>,
    output_value: Vec<&'a str>,
}

impl<'a> Pattern<'a> {
    fn new(data: &'a str) -> Self {
        let parsed_data = data
            .trim()
            .split('|')
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let unique_pattern = parsed_data[0].split(' ').collect::<Vec<&str>>();
        let output_value = parsed_data[1].split(' ').collect::<Vec<&str>>();
        Pattern {
            unique_pattern,
            output_value,
        }
    }

    // Count the unique alphabets and give a number between 0-9
    // Number 1: Has 2 letters
    // Number 4: Has 4 letters
    // Number 7: Has 3 letters
    // Number 8: Has 7 letters
    fn get_unique_number_from_str(data: &str) -> Option<u8> {
        match data.len() {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        }
    }
}

fn day8_part1(data: &Vec<Pattern>) -> u32 {
    data.iter()
        .map(|x| {
            x.output_value
                .iter()
                .map(|y| {
                    let ans = Pattern::get_unique_number_from_str(y);
                    match ans {
                        Some(_) => 1,
                        None => 0,
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_part1_test() {
        let data = [
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
            fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
            fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
            cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
            efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
            gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
            gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
            cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
            ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
            gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
            fgae cfgab fg bagce",
        ];

        let parsed_data = data
            .iter()
            .map(|x| Pattern::new(x))
            .collect::<Vec<Pattern>>();

        assert_eq!(parsed_data[0].unique_pattern[0], "be");
        assert_eq!(parsed_data[0].output_value[0], "fdgacbe");

        // Number 1: Has 2 letters
        // Number 4: Has 4 letters
        // Number 7: Has 3 letters
        // Number 8: Has 7 letters
        assert_eq!(Pattern::get_unique_number_from_str("fdgacbe").unwrap(), 8);
        assert_eq!(Pattern::get_unique_number_from_str("gcbe").unwrap(), 4);

        assert_eq!(day8_part1(&parsed_data), 26);
    }
}
