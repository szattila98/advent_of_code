use advent_of_code_2022::advent_task::{
    dummy_task::DummyTask, elven_calories::ElvenCalories, elven_tournament::ElvenTournament,
    rucksack_troubles::RucksackTroubles, AdventTask,
};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Dummy task", |b| b.iter(|| DummyTask.solve()));
    c.bench_function("Elven calories", |b| b.iter(|| ElvenCalories.solve()));
    c.bench_function("Elven tournament", |b| b.iter(|| ElvenTournament.solve()));
    c.bench_function("Rucksack troubles", |b| b.iter(|| RucksackTroubles.solve()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
