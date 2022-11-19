use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub mod dummy_task;

pub trait AdventTask<T: Display> {
    fn get_task_name(&self) -> String;
    fn get_input_file_name(&self) -> PathBuf;
    fn solve_first_part(&self, input: &[String]) -> T;
    fn solve_second_part(&self, input: &[String]) -> T;

    fn read_input(&self) -> Vec<String> {
        let file = File::open(self.get_input_file_name()).expect("could not open file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("could not parse line"))
            .collect()
    }

    fn solve(&self) {
        let font = neofiglet::FIGfont::standard().unwrap();
        let task_name = font.convert(&self.get_task_name()).unwrap();
        let input = self.read_input();
        let first_result = self.solve_first_part(&input);
        let second_result = self.solve_second_part(&input);
        println!("=========================================================================================");
        println!("{task_name}");
        println!("First result is - {}", first_result);
        println!("Second result is - {}", second_result);
        println!("=========================================================================================");
    }
}
