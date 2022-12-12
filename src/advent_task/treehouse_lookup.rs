use std::collections::HashSet;

use macros::include_str_arr;
use ndarray::{s, Array2, ArrayBase};

use super::AdventTask;

// TODO basic vec instead of ndarray for performance
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
        let mut visible_trees = 0;
        for ((row, col), tree) in forest.indexed_iter() {
            if row == 0 || row == forest.nrows() - 1 || col == 0 || col == forest.ncols() - 1 {
                visible_trees += 1;
                continue;
            }
            let row_of_tree = forest.row(row);
            // left
            if row_of_tree.slice(s![0..col]).iter().all(|num| num < tree) {
                visible_trees += 1;
                continue;
            }
            // right
            if row_of_tree
                .slice(s![col + 1..forest.ncols()])
                .iter()
                .all(|num| num < tree)
            {
                visible_trees += 1;
                continue;
            }

            let col_of_tree = forest.column(col);
            // top
            if col_of_tree.slice(s![0..row]).iter().all(|num| num < tree) {
                visible_trees += 1;
                continue;
            }
            // down
            if col_of_tree
                .slice(s![row + 1..forest.nrows()])
                .iter()
                .all(|num| num < tree)
            {
                visible_trees += 1;
                continue;
            }
        }
        visible_trees
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> Self::Solution {
        0
    }
}

fn parse_forest(
    input: &[Option<&'static str>],
) -> ArrayBase<ndarray::OwnedRepr<NumType>, ndarray::Dim<[usize; 2]>> {
    let mut forest = vec![];
    for line in input.iter().flatten() {
        let mut patch = vec![];
        for height in line.chars() {
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
