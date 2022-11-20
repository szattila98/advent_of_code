use std::fmt::Display;

pub mod dummy_task;

pub trait AdventTask<T: Display> {
    fn get_task_name(&self) -> &str;
    fn read_input_file(&self) -> &str;
    fn solve_first_part(&self, input: &[&str]) -> T;
    fn solve_second_part(&self, input: &[&str]) -> T;

    fn read_lines(&self) -> Vec<&str> {
        let file_content = self.read_input_file();
        file_content.lines().collect::<Vec<_>>()
    }

    fn solve(&self) -> (T, T) {
        let input = self.read_lines();
        let first_result = self.solve_first_part(&input);
        let second_result = self.solve_second_part(&input);
        (first_result, second_result)
    }
}
