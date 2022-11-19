use text_io::try_read;

use crate::advent_task::{dummy_task::DummyTask, AdventTask};

mod advent_task;

fn main() {
    println!("Hello, Advent of Code!");
    println!("Choose, which days code you want to run! (0-24)");
    let day = try_read!().expect("I can't understand you, you had one too many eggnogs friend!");
    match day {
        0 => DummyTask {}.solve(),
        _ => panic!("No such day, but happy holidays nonetheless!"),
    };
}
