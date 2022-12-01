use macros::include_str_arr;

use super::AdventTask;

pub struct DummyTask;

impl AdventTask<&'static str> for DummyTask {
    fn get_task_name(&self) -> &str {
        "Dummy Task"
    }

    fn get_inputs(&self) -> &[Option<&'static str>] {
        include_str_arr!("./inputs/dummy_input.txt")
    }

    fn solve_first_part(&self, input: &[Option<&'static str>]) -> &'static str {
        input
            .first()
            .expect("WHO INPUTTED THIS WRONG INPUT!")
            .expect("STOP IT!")
    }

    fn solve_second_part(&self, input: &[Option<&'static str>]) -> &'static str {
        input
            .last()
            .expect("STAAAAAAHP!")
            .expect("STRAIGHT TO JAIL, STRAIGHT AWAY!")
    }
}
