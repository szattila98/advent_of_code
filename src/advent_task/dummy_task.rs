use super::AdventTask;

pub struct DummyTask;

impl AdventTask<String> for DummyTask {
    fn read_input_file(&self) -> &str {
        include_str!("../../inputs/dummy_input.txt")
    }

    fn get_task_name(&self) -> &str {
        "Dummy Task"
    }

    fn solve_first_part(&self, input: &[&str]) -> String {
        input
            .first()
            .expect("WHO INPUTTED THIS WRONG INPUT!")
            .to_string()
    }

    fn solve_second_part(&self, input: &[&str]) -> String {
        input
            .last()
            .expect("STRAIGHT TO JAIL, STRAIGHT AWAY!")
            .to_string()
    }
}
