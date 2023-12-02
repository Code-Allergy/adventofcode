use default::{Answer, Solution};

#[derive(Debug)]
pub struct Day01;

fn extract_numeric_chars(line: &str) -> Vec<char> {
    line.chars().filter(|&c| c.is_numeric()).collect()
}

impl Solution for Day01 {
    fn name(&self) -> &'static str {
        "Trebuchet?!"
    }

    fn p1(&self, input: &str) -> Answer {
        let num_vecs: Vec<Vec<char>> = input
            .lines()
            .map(|line| extract_numeric_chars(line))
            .collect();

        let sum: i32 = num_vecs
            .iter()
            .map(|char_vec| {
                if let (Some(&char1), Some(&char2)) = (char_vec.first(), char_vec.last()) {
                    let number: i32 = format!("{}{}", char1, char2)
                        .parse()
                        .expect("Result is non-numeric!");
                    number
                } else {
                    panic!("Malformed line!")
                }
            })
            .sum();
        Answer::from(sum)
    }

    fn p2(&self, input: &str) -> Answer {
        // println!("{input}");
        return Answer::Unimplemented;
    }
}

#[cfg(test)]
mod test {
    use super::Day01;
    use default::Solution;
    use indoc::indoc;

    const PART1_SAMPLE: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    const PART2_SAMPLE: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn test_1() {
        assert_eq!(Day01.p1(PART1_SAMPLE), 142.into());
    }

    #[test]
    fn test_2() {
        assert_eq!(Day01.p2(PART2_SAMPLE), 281.into());
    }
}
