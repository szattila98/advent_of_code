use std::collections::HashSet;

use macros::include_str_arr;
use ndarray::{s, Array2, ArrayBase};

use super::AdventTask;

pub struct TreeHouseLookup;

type NumType = usize;

impl AdventTask for TreeHouseLookup {
    type Solution = NumType;

    fn get_name(&self) -> &str {
        "Treetop Lookup"
    }

    fn get_inputs(&self) -> &[Option<&'static str>] {
        include_str_arr!("./inputs/treehouse_lookup.txt")
    }

    fn solve_first_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        let forest = parse_forest(input);
        let mut visible_trees = HashSet::new();
        for ((row, col), tree) in forest.indexed_iter() {
            if row == 0 || row == forest.nrows() - 1 || col == 0 || col == forest.ncols() - 1 {
                visible_trees.insert((row, col, *tree));
            }
            // left
            if dbg!(forest.slice(s![0..row, 0..col]))
                .iter()
                .all(|num| num < tree)
            {
                visible_trees.insert((row, col, *tree));
                continue;
            }
            // right
            // for i in col + 1..forest.ncols() {
            //     if forest.get((row, i)).unwrap() < tree {
            //         visible_trees.insert((row, col, *tree));
            //         continue;
            //     }
            // }
            // top
            // for i in 0..row {
            //     if forest.get((i, col)).unwrap() < tree {
            //         visible_trees.insert((row, col, *tree));
            //         continue;
            //     }
            // }
            // down
            // for i in row + 1..forest.nrows() {
            //     if forest.get((i, col)).unwrap() < tree {
            //         visible_trees.insert((row, col, *tree));
            //         continue;
            //     }
            // }
        }
        visible_trees.len() // - 4
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        0
    }
}

fn parse_forest(
    input: &[Option<&'static str>],
) -> ArrayBase<ndarray::OwnedRepr<NumType>, ndarray::Dim<[usize; 2]>> {
    let mut forest = vec![];
    for (y, line) in input.iter().flatten().enumerate() {
        let mut patch = vec![];
        for (x, height) in line.chars().enumerate() {
            patch.push(height.to_digit(10).unwrap() as NumType)
        }
        forest.push(patch);
    }
    let mut data = Vec::new();
    let ncols = forest.first().map_or(0, |row| row.len());
    let mut nrows = 0;
    for i in 0..forest.len() {
        data.extend_from_slice(&forest[i]);
        nrows += 1;
    }
    Array2::from_shape_vec((nrows, ncols), data).unwrap()
}
