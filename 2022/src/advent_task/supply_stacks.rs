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
        let (mut stacks, moves) = parse_input(input);
        for m in moves {
            for _ in 0..m.count {
                let source = &mut stacks[m.from - 1];
                let supply_chest = source.pop().unwrap();
                let destination = &mut stacks[m.to - 1];
                destination.push(supply_chest);
            }
        }
        stacks.iter().map(|stack| stack.last().unwrap()).collect()
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let (mut stacks, moves) = parse_input(input);
        for m in moves {
            for i in (0..m.count).rev() {
                let source = &mut stacks[m.from - 1];
                let index = source.len() - i - 1;
                let supplies = source[index];
                source.remove(index);
                let destination = &mut stacks[m.to - 1];
                destination.push(supplies);
            }
        }
        stacks.iter().map(|stack| stack.last().unwrap()).collect()
    }
}

fn parse_input(input: &[Option<&str>]) -> (Vec<Vec<char>>, Vec<Move>) {
    let divider_index = input.iter().position(|line| line.is_none()).unwrap();
    let stacks = parse_stacks(input, divider_index);
    let moves = parse_moves(input, divider_index);
    (stacks, moves)
}

fn parse_stacks(input: &[Option<&str>], divider_index: usize) -> Vec<Vec<char>> {
    input[divider_index - 1]
        .unwrap()
        .trim()
        .split("   ")
        .map(|s| {
            (0..divider_index - 1)
                .rev()
                .filter_map(|j| {
                    let content = input[j]
                        .unwrap()
                        .chars()
                        .nth(1 + (s.parse::<usize>().unwrap() - 1) * 4)
                        .unwrap();
                    if content != ' ' {
                        Some(content)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_moves(input: &[Option<&str>], divider_index: usize) -> Vec<Move> {
    input[divider_index + 1..]
        .iter()
        .flatten()
        .map(|line| {
            let numbers = line
                .split(' ')
                .filter_map(|word| match word.parse::<usize>() {
                    Ok(n) => Some(n),
                    Err(_) => None,
                })
                .collect::<Vec<_>>();
            Move {
                count: numbers[0],
                from: numbers[1],
                to: numbers[2],
            }
        })
        .collect::<Vec<_>>()
}
