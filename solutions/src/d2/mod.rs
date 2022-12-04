use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// A X  rock
// B Y  paper
// C Z  scissors
//
// win      6
// draw     3
// defeat   0
//

enum GameResult {
    Win,
    Defeat,
    Draw,
}
impl GameResult {
    fn get_points(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Defeat => 0,
        }
    }
}
impl GameResult {
    fn from_str(s: &str) -> Option<GameResult> {
        match s {
            "X" => Some(GameResult::Defeat),
            "Y" => Some(GameResult::Draw),
            "Z" => Some(GameResult::Win),
            _ => None,
        }
    }
    fn get_move_for_result(&self, mv: MoveOption) -> MoveOption {
        match self {
            Self::Win => match mv {
                MoveOption::Rock => MoveOption::Paper,
                MoveOption::Paper => MoveOption::Scissors,
                MoveOption::Scissors => MoveOption::Rock,
            },
            Self::Draw => mv,
            Self::Defeat => match mv {
                MoveOption::Rock => MoveOption::Scissors,
                MoveOption::Paper => MoveOption::Rock,
                MoveOption::Scissors => MoveOption::Paper,
            },
        }
    }
}

enum MoveOption {
    Rock,
    Paper,
    Scissors,
}
impl MoveOption {
    fn from_str(s: &str) -> Option<MoveOption> {
        match s {
            "A" | "X" => Some(MoveOption::Rock),
            "B" | "Y" => Some(MoveOption::Paper),
            "C" | "Z" => Some(MoveOption::Scissors),
            _ => None,
        }
    }
    fn get_result(&self, other: &Self) -> GameResult {
        match self {
            Self::Rock => match other {
                Self::Rock => GameResult::Draw,
                Self::Paper => GameResult::Defeat,
                Self::Scissors => GameResult::Win,
            },
            Self::Paper => match other {
                Self::Rock => GameResult::Win,
                Self::Paper => GameResult::Draw,
                Self::Scissors => GameResult::Defeat,
            },
            Self::Scissors => match other {
                Self::Rock => GameResult::Defeat,
                Self::Paper => GameResult::Win,
                Self::Scissors => GameResult::Draw,
            },
        }
    }
    fn get_move_points(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

pub fn do_first_part(file_path: &str) -> u32 {
    let f = File::open(file_path).expect("failed to open file");

    BufReader::new(f)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            let mut parsed = l
                .split_whitespace()
                .map(|m| MoveOption::from_str(&m))
                .filter_map(|m| m);

            (parsed.next().unwrap(), parsed.next().unwrap())
        })
        .map(|moves| moves.1.get_result(&moves.0).get_points() + moves.1.get_move_points())
        .fold(0, |acc, r| acc + r)
}

pub fn do_sec_part(file_path: &str) -> u32 {
    let f = File::open(file_path).expect("failed to open file");

    BufReader::new(f)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            let mut splitted = l.split_whitespace();
            (
                MoveOption::from_str(splitted.next().unwrap()).unwrap(),
                GameResult::from_str(splitted.next().unwrap()).unwrap(),
            )
        })
        .map(|parsed| {
            parsed.1.get_move_for_result(parsed.0).get_move_points() + parsed.1.get_points()
        })
        .fold(0, |acc, r| acc + r)
}

#[cfg(test)]
mod d2_tests {
    use super::*;

    #[test]
    fn test_first() {
        let res = do_first_part("./src/d2/test.txt");

        assert_eq!(res, 15);
    }

    #[test]
    fn test_second() {
        let res = do_sec_part("./src/d2/test.txt");

        assert_eq!(res, 12);
    }
}
