use super::AdventTask;

pub struct DummyTask;

impl AdventTask<String> for DummyTask {
    fn get_inputs(&self) -> &str {
        include_str!("../../inputs/dummy_input.txt")
    }

    fn get_task_name(&self) -> &str {
        "Dummy Task"
    }

    fn solve_first_part(&self, input: &[Option<&str>]) -> String {
        input
            .first()
            .expect("WHO INPUTTED THIS WRONG INPUT!")
            .expect("STOP IT!")
            .to_string()
    }

    fn solve_second_part(&self, input: &[Option<&str>]) -> String {
        input
            .last()
            .expect("STAAAAAAHP!")
            .expect("STRAIGHT TO JAIL, STRAIGHT AWAY!")
            .to_string()
    }
}
