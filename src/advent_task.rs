use std::fmt::Display;

pub mod dummy_task;
pub mod elven_calories;

pub trait AdventTask<T: Display> {
    fn get_task_name(&self) -> &str;
    fn get_inputs(&self) -> &[Option<&'static str>];
    fn solve_first_part(&self, input: &[Option<&'static str>]) -> T;
    fn solve_second_part(&self, input: &[Option<&'static str>]) -> T;

    fn solve(&self) -> (T, T) {
        let input_arr = self.get_inputs();
        let first_result = self.solve_first_part(&input_arr);
        let second_result = self.solve_second_part(&input_arr);
        (first_result, second_result)
    }
}
