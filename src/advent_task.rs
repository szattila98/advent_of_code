use std::fmt::Display;

pub mod dummy_task;

const INPUT_SIZE: usize = 100;

pub trait AdventTask<T: Display> {
    fn get_task_name(&self) -> &str;
    fn get_inputs(&self) -> &str;
    fn solve_first_part(&self, input: &[Option<&str>]) -> T;
    fn solve_second_part(&self, input: &[Option<&str>]) -> T;

    fn solve(&self) -> (T, T) {
        let input = self.get_inputs();
        let mut input_arr: [Option<&str>; INPUT_SIZE] = [None; INPUT_SIZE];
        input.lines().enumerate().for_each(|(i, x)| {
            if i >= INPUT_SIZE {
                panic!("input files must only contain {INPUT_SIZE} lines or less")
            }
            input_arr[i] = Some(x);
        });
        let first_result = self.solve_first_part(&input_arr);
        let second_result = self.solve_second_part(&input_arr);
        (first_result, second_result)
    }
}
