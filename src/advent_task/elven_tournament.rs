use macros::include_str_arr;

use super::AdventTask;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSS: u32 = 0;
const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

pub struct ElvenTournament;

impl AdventTask<u32> for ElvenTournament {
    fn get_task_name(&self) -> &str {
        "Elven Tournament"
    }

    fn get_inputs(&self) -> &[Option<&'static str>] {
        include_str_arr!("./inputs/elven_tournament.txt")
    }

    fn solve_first_part(&self, input: &[Option<&'static str>]) -> u32 {
        let mut score = 0;
        let matches = input.iter().flatten().collect::<Vec<_>>();
        for m in matches {
            match *m {
                "A X" => score += ROCK + DRAW,
                "A Y" => score += PAPER + WIN,
                "A Z" => score += SCISSORS + LOSS,
                "B X" => score += ROCK + LOSS,
                "B Y" => score += PAPER + DRAW,
                "B Z" => score += SCISSORS + WIN,
                "C X" => score += ROCK + WIN,
                "C Y" => score += PAPER + LOSS,
                "C Z" => score += SCISSORS + DRAW,
                _ => panic!("what trickery is this?"),
            }
        }
        score
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> u32 {
        let mut score = 0;
        let matches = input.iter().flatten().collect::<Vec<_>>();
        for m in matches {
            match *m {
                "A X" => score += SCISSORS + LOSS,
                "A Y" => score += ROCK + DRAW,
                "A Z" => score += PAPER + WIN,
                "B X" => score += ROCK + LOSS,
                "B Y" => score += PAPER + DRAW,
                "B Z" => score += SCISSORS + WIN,
                "C X" => score += PAPER + LOSS,
                "C Y" => score += SCISSORS + DRAW,
                "C Z" => score += ROCK + WIN,
                _ => panic!("what trickery is this?"),
            }
        }
        score
    }
}
