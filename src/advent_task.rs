use std::fmt::Display;

pub mod camp_cleaning;
pub mod elven_calories;
pub mod elven_tournament;
pub mod rucksack_troubles;
pub mod supply_stacks;
pub mod tuning_trouble;

pub trait AdventTask {
    type Solution: Display;

    fn get_name(&self) -> &str;
    fn get_inputs(&self) -> &[Option<&'static str>];
    fn solve_first_part(&self, input: &[Option<&'static str>]) -> Self::Solution;
    fn solve_second_part(&self, input: &[Option<&'static str>]) -> Self::Solution;

    fn solve(&self) -> (Self::Solution, Self::Solution) {
        let input_arr = self.get_inputs();
        let first_result = self.solve_first_part(input_arr);
        let second_result = self.solve_second_part(input_arr);
        (first_result, second_result)
    }
}
