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
            let mut to_be_moved = vec![];
            for _ in 0..m.count {
                let stack = &mut stacks[m.from - 1];
                to_be_moved.push(stack.pop().unwrap());
            }
            for i in to_be_moved {
                stacks[m.to - 1].push(i)
            }
        }
        let mut result = String::from("");
        stacks
            .iter()
            .for_each(|stack| result = format!("{result}{}", stack.last().unwrap()));
        result
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let (mut stacks, moves) = parse_input(input);
        for m in moves {
            let mut to_be_moved = vec![];
            for _ in 0..m.count {
                let stack = &mut stacks[m.from - 1];
                to_be_moved.push(stack.pop().unwrap());
            }
            to_be_moved.reverse();
            for i in to_be_moved {
                stacks[m.to - 1].push(i)
            }
        }
        let mut result = String::from("");
        stacks
            .iter()
            .for_each(|stack| result = format!("{result}{}", stack.last().unwrap()));
        result
    }
}

fn parse_input(input: &[Option<&str>]) -> (Vec<Vec<char>>, Vec<Move>) {
    let divider_index = input.iter().position(|line| line.is_none()).unwrap();
    let stacks = parse_stacks(input, divider_index);
    let moves = parse_moves(input, divider_index);
    (stacks, moves)
}

fn parse_stacks(input: &[Option<&str>], divider_index: usize) -> Vec<Vec<char>> {
    let mut stacks = vec![];
    input[divider_index - 1]
        .unwrap()
        .trim()
        .split("   ")
        .enumerate()
        .for_each(|(i, _)| {
            let mut contents = vec![];
            for j in (0..divider_index - 1).rev() {
                let content = input[j].unwrap().chars().nth(1 + i * 4).unwrap();
                if content != ' ' {
                    contents.push(content)
                }
            }
            stacks.push(contents);
        });
    stacks
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
