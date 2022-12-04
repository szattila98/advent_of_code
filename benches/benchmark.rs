use advent_of_code_2022::advent_task::{
    camp_cleaning::CampCleaning, dummy_task::DummyTask, elven_calories::ElvenCalories,
    elven_tournament::ElvenTournament, rucksack_troubles::RucksackTroubles, AdventTask,
};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(DummyTask.get_task_name(), |b| b.iter(|| DummyTask.solve()));
    c.bench_function(ElvenCalories.get_task_name(), |b| {
        b.iter(|| ElvenCalories.solve())
    });
    c.bench_function(ElvenTournament.get_task_name(), |b| {
        b.iter(|| ElvenTournament.solve())
    });
    c.bench_function(RucksackTroubles.get_task_name(), |b| {
        b.iter(|| RucksackTroubles.solve())
    });
    c.bench_function(CampCleaning.get_task_name(), |b| {
        b.iter(|| CampCleaning.solve())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
