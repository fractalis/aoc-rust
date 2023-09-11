use std::str::FromStr;

#[derive(Debug)]
enum RPSParseError {
    InvalidInput,
}

#[derive(Debug)]
enum RPSEnum {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum RPSPlayer {
    Player1,
    Player2,
    Draw,
}

#[derive(Debug)]
struct RPSRound {
    player1: RPSEnum,
    player2: RPSEnum,
}

impl FromStr for RPSRound {
    type Err = RPSParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_at(s.find(' ').ok_or(RPSParseError::InvalidInput)?);
        Ok(RPSRound {
            player1: match p1.trim() {
                "A" => RPSEnum::Rock,
                "B" => RPSEnum::Paper,
                "C" => RPSEnum::Scissors,
                _ => return Err(RPSParseError::InvalidInput),
            },
            player2: match p2.trim() {
                "X" => RPSEnum::Rock,
                "Y" => RPSEnum::Paper,
                "Z" => RPSEnum::Scissors,
                _ => return Err(RPSParseError::InvalidInput),
            },
        })
    }
}

pub fn calculate_score(input: &str) -> (RPSPlayer, u32) {
    let round = RPSRound::from_str(input).unwrap();

    let (winner, score) = match (round.player1, round.player2) {
        (RPSEnum::Rock, RPSEnum::Scissors) => (RPSPlayer::Player1, 3),
        (RPSEnum::Rock, RPSEnum::Paper) => (RPSPlayer::Player2, 8),
        (RPSEnum::Paper, RPSEnum::Rock) => (RPSPlayer::Player1, 1),
        (RPSEnum::Paper, RPSEnum::Scissors) => (RPSPlayer::Player2, 9),
        (RPSEnum::Scissors, RPSEnum::Paper) => (RPSPlayer::Player1, 2),
        (RPSEnum::Scissors, RPSEnum::Rock) => (RPSPlayer::Player2, 7),
        (RPSEnum::Rock, RPSEnum::Rock) => (RPSPlayer::Draw, 4),
        (RPSEnum::Scissors, RPSEnum::Scissors) => (RPSPlayer::Draw, 6),
        (RPSEnum::Paper, RPSEnum::Paper) => (RPSPlayer::Draw, 5),
    };

    (winner, score)
}

pub fn part_one(input: &str) -> Option<u32> {
    let res: u32 = input.lines().map(|line| calculate_score(line).1).sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
