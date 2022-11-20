use std::fmt::Display;

use text_io::try_read;

use advent_of_code_2022::advent_task::{dummy_task::DummyTask, AdventTask};

fn main() {
    println!("Hello, Advent of Code!");
    println!("Choose, which days code you want to run! (0-24)");
    let day = try_read!().expect("I can't understand you, you had one too many eggnogs friend!");
    match day {
        0 => print_solution(DummyTask {}),
        _ => println!("No such day, but happy holidays nonetheless!"),
    };
}

fn print_solution<R: Display, T: AdventTask<R>>(task: T) {
    let font = neofiglet::FIGfont::standard().unwrap();
    let task_name = font.convert(&task.get_task_name()).unwrap();
    let (first_result, second_result) = task.solve();
    println!("================================================================================");
    println!("{task_name}");
    println!("================================================================================");
    println!("First result is - {}", first_result);
    println!("Second result is - {}", second_result);
    println!("================================================================================");
}
