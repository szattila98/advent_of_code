use std::path::PathBuf;

use super::AdventTask;

pub struct DummyTask;

impl AdventTask<String> for DummyTask {
    fn read_input(&self) -> Vec<String> {
        vec!["Hello".to_string(), "Advent".to_string()]
    }

    fn get_task_name(&self) -> String {
        "Dummy Task".to_string()
    }

    fn get_input_file_name(&self) -> PathBuf {
        "./dummy/file/name/never/to/be/used"
            .parse()
            .expect("WHY COULDN'T YOU PARSE THE DUMMY FILE PATH DUMMY")
    }

    fn solve_first_part(&self, input: &[String]) -> String {
        input
            .get(0)
            .expect("WHO INPUTTED THIS WRONG INPUT!")
            .to_owned()
    }

    fn solve_second_part(&self, input: &[String]) -> String {
        input
            .get(1)
            .expect("STRAIGHT TO JAIL, STRAIGHT AWAY!")
            .to_owned()
    }
}
