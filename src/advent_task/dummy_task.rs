use super::AdventTask;

pub struct DummyTask;

impl AdventTask<&'static str> for DummyTask {
    fn get_inputs(&self) -> &str {
        include_str!("../../inputs/dummy_input.txt")
    }

    fn get_task_name(&self) -> &str {
        "Dummy Task"
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
