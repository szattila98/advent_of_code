use macros::include_str_arr;

use super::AdventTask;

type SolutionType = u16;

const WIN: SolutionType = 6;
const DRAW: SolutionType = 3;
const LOSS: SolutionType = 0;
const ROCK: SolutionType = 1;
const PAPER: SolutionType = 2;
const SCISSORS: SolutionType = 3;

pub struct ElvenTournament;

impl AdventTask for ElvenTournament {
    type Solution = SolutionType;

    fn get_name(&self) -> &str {
        "Elven Tournament"
    }

    fn get_inputs(&self) -> &[Option<&'static str>] {
        include_str_arr!("./inputs/elven_tournament.txt")
    }

    fn solve_first_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let mut score = 0;
        for m in input.iter().flatten() {
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

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let mut score = 0;
        for m in input.iter().flatten() {
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
