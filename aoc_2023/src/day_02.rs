use default::{Answer, Solution};
pub struct Day02;

//You're launched high into the atmosphere! The apex of your trajectory just barely reaches the
// surface of a large island floating in the sky. You gently land in a fluffy pile of leaves.
// It's quite cold, but you don't see much snow. An Elf runs over to greet you.
//
// The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow.
// He'll be happy to explain the situation, but it's a bit of a walk, so you have some time.
// They don't get many visitors up here; would you like to play a game in the meantime?
//
// As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue.
// Each time you play this game, he will hide a secret number of cubes of each color in the bag,
// and your goal is to figure out information about the number of cubes.
//
// To get information, once a bag has been loaded with cubes, the Elf will reach into the bag,
// grab a handful of random cubes, show them to you, and then put them back in the bag.
// He'll do this a few times per game.
//
// You play several games and record the information from each game (your puzzle input).
// Each game is listed with its ID number (like the 11 in Game 11: ...)
// followed by a semicolon-separated list of subsets of cubes that were revealed from the bag
// (like 3 red, 5 green, 4 blue).
//
// For example, the record of a few games might look like this:
//
//
// In game 1, three sets of cubes are revealed from the bag (and then put back again).
// The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes,
// and 6 blue cubes; the third set is only 2 green cubes.
//
// The Elf would first like to know which games would have been possible if the bag contained
// only 12 red cubes, 13 green cubes, and 14 blue cubes?
//
// In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded
// with that configuration. However, game 3 would have been impossible because at one point the
// Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because
// the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have
// been possible, you get 8.
//
// Determine which games would have been possible if the bag had been loaded with only 12 red
// cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?

#[derive(Debug)]
struct Round {
    red: Option<u32>,
    blue: Option<u32>,
    green: Option<u32>,
}

impl Round {
    fn merge_max(&self, other: &Round) -> Round {
        Round {
            red: self.red.map_or_else(
                || other.red,
                |v| other.red.map_or_else(|| Some(v), |w| Some(v.max(w))),
            ),
            blue: self.blue.map_or_else(
                || other.blue,
                |v| other.blue.map_or_else(|| Some(v), |w| Some(v.max(w))),
            ),
            green: self.green.map_or_else(
                || other.green,
                |v| other.green.map_or_else(|| Some(v), |w| Some(v.max(w))),
            ),
        }
    }
}

const RULES: Round = Round {
    red: Some(12),
    blue: Some(14),
    green: Some(13),
};

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
    game_no: u32,
}

fn parse_line_to_game(game_line: &str) -> Game {
    let (game_no, games) = game_line.split_once(": ").expect("Malformed line!");
    let games: Vec<&str> = games.split("; ").collect();
    let game_no: u32 = game_no
        .split(" ")
        .nth(1)
        .unwrap()
        .parse()
        .expect("Malformed line!");
    let mut game = Game {
        rounds: vec![],
        game_no,
    };
    for game_str in &games {
        let mut round = Round {
            red: None,
            blue: None,
            green: None,
        };
        let game_str: Vec<&str> = game_str.split(", ").collect();
        for hand in game_str {
            let (count, color) = hand.split_once(" ").expect("Malformed line!");
            let count = count.parse::<u32>();
            match count {
                Ok(count) => match color {
                    "red" => round.red = Some(count),
                    "green" => round.green = Some(count),
                    "blue" => round.blue = Some(count),
                    _ => {}
                },
                _ => {}
            }
        }
        game.rounds.push(round);
    }
    game
}

fn matches_rules(game: &Game) -> bool {
    for round in &game.rounds {
        if round.red > RULES.red || round.green > RULES.green || round.blue > RULES.blue {
            return false;
        }
    }
    true
}

fn get_round_square(max: &Round) -> u32 {
    max.blue.unwrap() * max.green.unwrap() * max.red.unwrap()
}

impl Solution for Day02 {
    fn name(&self) -> &'static str {
        "Cube Conundrum"
    }

    fn p1(&self, input: &str) -> Answer {
        let mut sum = 0;
        for line in input.lines() {
            let game = parse_line_to_game(line);
            if matches_rules(&game) {
                sum += game.game_no;
            }
        }
        Answer::from(sum)
    }

    fn p2(&self, input: &str) -> Answer {
        let mut sum = 0;
        for line in input.lines() {
            let game = parse_line_to_game(line);

            let max_round = game.rounds.iter().fold(
                Round {
                    red: None,
                    blue: None,
                    green: None,
                },
                |acc, round| acc.merge_max(round),
            );
            sum += get_round_square(&max_round);
        }
        Answer::from(sum)
    }
}

#[cfg(test)]
mod test {
    use crate::day_02::Day02;
    use default::Solution;
    use indoc::indoc;

    const PART1_SAMPLE: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn test_1() {
        assert_eq!(Day02.p1(PART1_SAMPLE), 8.into());
    }
    #[test]
    fn test_2() {
        assert_eq!(Day02.p2(PART1_SAMPLE), 2286.into());
    }
}
