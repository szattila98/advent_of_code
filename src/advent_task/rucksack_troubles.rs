use macros::include_str_arr;

use super::AdventTask;

const LOWERCASE_OFFSET: u32 = 96;
const UPPERCASE_OFFSET: u32 = 38;

pub struct RucksackTroubles;

impl AdventTask<u32> for RucksackTroubles {
    fn get_task_name(&self) -> &str {
        "Rucksack Troubles"
    }

    fn get_inputs(&self) -> &[Option<&'static str>] {
        include_str_arr!("./inputs/rucksack_troubles.txt")
    }

    fn solve_first_part(&self, input: &[Option<&'static str>]) -> u32 {
        let mut priorities = 0;
        input.iter().flatten().for_each(|input| {
            let half_len = input.len() / 2;
            let first_compartment = &input[..half_len];
            let second_compartment = &input[half_len..];
            for item in first_compartment.chars() {
                if second_compartment.contains(item) {
                    priorities += calculate_priority(item);
                    break;
                }
            }
        });
        priorities
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> u32 {
        let group_count = input.len() / 3;
        let mut priorities = 0;
        for i in 0..group_count {
            let first_rucksack = input[i * 3].expect("no rucksack");
            let second_rucksack = input[i * 3 + 1].expect("why no rucksack");
            let third_rucksack = input[i * 3 + 2].expect("gib rucksack");
            for item in first_rucksack.chars() {
                if second_rucksack.contains(item) && third_rucksack.contains(item) {
                    priorities += calculate_priority(item);
                    break;
                }
            }
        }
        priorities
    }
}

fn calculate_priority(item: char) -> u32 {
    if item.is_lowercase() {
        item as u32 - LOWERCASE_OFFSET
    } else {
        item as u32 - UPPERCASE_OFFSET
    }
}
