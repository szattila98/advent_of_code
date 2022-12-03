use std::fmt::Display;

use text_io::try_read;

use advent_of_code_2022::advent_task::{
    dummy_task::DummyTask, elven_calories::ElvenCalories, elven_tournament::ElvenTournament,
    rucksack_troubles::RucksackTroubles, AdventTask,
};

fn main() {
    println!("Hello, Advent of Code!");
    println!("Choose, which days code you want to run! (0-24)");
    let day = try_read!().expect("I can't understand you, you had one too many eggnogs friend!");
    match day {
        0 => print_solution(DummyTask),
        1 => print_solution(ElvenCalories),
        2 => print_solution(ElvenTournament),
        3 => print_solution(RucksackTroubles),
        _ => panic!("No such day, but happy holidays nonetheless!"),
    };
}

fn print_solution<R: Display, T: AdventTask<R>>(task: T) {
    let font = neofiglet::FIGfont::standard().unwrap();
    let task_name = font.convert(task.get_task_name()).unwrap();
    let (first_result, second_result) = task.solve();
    println!("================================================================================");
    println!("{task_name}");
    println!("================================================================================");
    println!("First solution - {}", first_result);
    println!("Second solution - {}", second_result);
    println!("================================================================================");
}
