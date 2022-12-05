use std::{collections::HashMap, fmt::format};

use macros::include_str_arr;

use super::AdventTask;

pub struct SupplyStacks;

impl AdventTask for SupplyStacks {
    type Solution = String;

    fn get_name(&self) -> &str {
        "Supply Stacks"
    }

    fn get_inputs(&self) -> &[Option<&'static str>] {
        include_str_arr!("./inputs/supply_stacks.txt")
    }

    fn solve_first_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let mut stacks = parse_stacks(input);
        format!("{stacks:?}")
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        "2".to_string()
    }
}

fn parse_stacks(input: &[Option<&str>]) -> HashMap<usize, Vec<char>> {
    let mut stacks = HashMap::new();
    let divider_index = input.iter().position(|line| line.is_none()).unwrap();
    input[divider_index - 1]
        .unwrap()
        .trim()
        .split("   ")
        .enumerate()
        .for_each(|(i, stack)| {
            let stack = stack.parse().unwrap();
            let mut contents = vec![];
            for j in (0..divider_index - 1).rev() {
                let content = input[j].unwrap().chars().nth(1 + i * 4).unwrap();
                if content != ' ' {
                    contents.push(content)
                }
            }
            stacks.insert(stack, contents);
        });
    stacks
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}
