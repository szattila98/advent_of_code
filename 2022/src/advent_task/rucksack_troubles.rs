use macros::include_str_arr;

use super::AdventTask;

type SolutionType = u16;

const LOWERCASE_OFFSET: SolutionType = 96;
const UPPERCASE_OFFSET: SolutionType = 38;

pub struct RucksackTroubles;

impl AdventTask for RucksackTroubles {
    type Solution = SolutionType;

    fn get_name(&self) -> &str {
        "Rucksack Troubles"
    }

    fn get_inputs(&self) -> &[Option<&'static str>] {
        include_str_arr!("./inputs/rucksack_troubles.txt")
    }

    fn solve_first_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let mut priorities = 0;
        for input in input.iter().flatten() {
            let half_len = input.len() / 2;
            let first_compartment = &input[..half_len];
            let second_compartment = &input[half_len..];
            for item in first_compartment.chars() {
                if second_compartment.contains(item) {
                    priorities += Self::calculate_priority(item);
                    break;
                }
            }
        }
        priorities
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let group_count = input.len() / 3;
        let mut priorities = 0;
        for i in 0..group_count {
            let first_rucksack = input[i * 3].expect("no rucksack");
            let second_rucksack = input[i * 3 + 1].expect("why no rucksack");
            let third_rucksack = input[i * 3 + 2].expect("gib rucksack");
            for item in first_rucksack.chars() {
                if second_rucksack.contains(item) && third_rucksack.contains(item) {
                    priorities += Self::calculate_priority(item);
                    break;
                }
            }
        }
        priorities
    }
}

impl RucksackTroubles {
    fn calculate_priority(item: char) -> <RucksackTroubles as AdventTask>::Solution {
        if item.is_lowercase() {
            item as SolutionType - LOWERCASE_OFFSET
        } else {
            item as SolutionType - UPPERCASE_OFFSET
        }
    }
}
